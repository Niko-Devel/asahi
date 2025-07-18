use {
  crate::{
    canvas::{
      EmoteSource,
      fetch_discord_emote,
      fetch_twemoji_emote
    },
    templates::DISCORD_EMOTES_CACHE
  },
  std::sync::{
    LazyLock,
    mpsc::{
      self,
      Sender
    }
  }
};

pub(crate) static EMOTE_FETCHER_TX: LazyLock<Sender<EmoteSource>> = LazyLock::new(spawn_emote_fetcher);

/// Notify the background worker to refresh the cache with incoming emotes
pub fn prefetch_emotes<I: IntoIterator<Item = EmoteSource>>(emotes: I) {
  for emote in emotes {
    let _ = EMOTE_FETCHER_TX.send(emote);
  }
}

fn spawn_emote_fetcher() -> Sender<EmoteSource> {
  use std::thread;
  let (tx, rx) = mpsc::channel::<EmoteSource>();

  thread::spawn(move || {
    let rt = tokio::runtime::Runtime::new().unwrap();
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    rt.block_on(async move {
      while let Ok(emote) = rx.recv() {
        if !seen.insert(emote.clone()) {
          continue;
        }
        match emote {
          EmoteSource::Discord(ref id) => {
            if let Some(img) = fetch_discord_emote(id).await {
              DISCORD_EMOTES_CACHE.lock().unwrap().insert(id.clone(), img);
            }
          },
          EmoteSource::Unicode(ch) => {
            let key = format!("twemoji_{}", ch as u32);
            if let Some(img) = fetch_twemoji_emote(ch).await {
              DISCORD_EMOTES_CACHE.lock().unwrap().insert(key, img);
            }
          }
        }
      }
    });
  });

  tx
}
