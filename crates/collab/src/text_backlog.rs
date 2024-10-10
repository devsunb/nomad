use collab_project::file::Text;
use collab_project::PeerId;
use nohash::IntMap as NoHashMap;
use smol_str::SmolStr;

#[derive(Default)]
pub(crate) struct TextBacklog {
    map: NoHashMap<PeerId, PeerBacklog>,
}

impl TextBacklog {
    pub(crate) fn insert(&mut self, text: Text, s: SmolStr) {
        assert!(text.char_range().len() <= s.len());

        self.map
            .entry(text.inserted_by())
            .or_default()
            .insert(text.char_range().start, s);
    }

    pub(crate) fn remove(&mut self, text: Text) -> SmolStr {
        let Some(inner) = self.map.get_mut(&text.inserted_by()) else {
            panic!("no backlog for peer");
        };
        inner.remove(text.char_range().start)
    }
}

#[derive(Default)]
struct PeerBacklog {
    vec: Vec<(SmolStr, usize)>,
}

impl PeerBacklog {
    fn insert(&mut self, offset: usize, text: SmolStr) {
        let Err(insert_idx) = self
            .vec
            .binary_search_by(|(_, existing_off)| existing_off.cmp(&offset))
        else {
            panic!("text already exists at offset {offset}");
        };

        assert!(insert_idx
            .checked_sub(1)
            .and_then(|idx| self.vec.get(idx))
            .map(|prev| prev.1 + prev.0.len() <= offset)
            .unwrap_or(true));

        assert!(self
            .vec
            .get(insert_idx)
            .map(|next| offset + text.len() <= next.1)
            .unwrap_or(true));

        self.vec.insert(insert_idx, (text, offset));
    }

    fn remove(&mut self, offset: usize) -> SmolStr {
        let Ok(remove_idx) = self
            .vec
            .binary_search_by(|(_, existing_off)| existing_off.cmp(&offset))
        else {
            panic!("no text at offset {offset}");
        };

        self.vec.remove(remove_idx).0
    }
}
