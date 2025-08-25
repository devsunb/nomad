use abs_path::path;
use collab_project::text::TextReplacement;
use collab_project::{PeerId, Project};
use mock::fs::MockFs;

#[test]
fn roundtrip_empty() {
    let project = Project::new(PeerId::new(1));
    let encoded = project.encode();
    let decoded = Project::decode(&encoded, PeerId::new(2)).unwrap();
    assert_eq!(MockFs::from(&project).root(), MockFs::from(&decoded).root());
}

#[test]
fn edit_text_after_roundtrip() {
    let fs = mock::fs! {
        "foo.txt": "hello world",
    };
    let project = Project::from_mock(PeerId::new(1), fs.root());
    let encoded = project.encode();
    let mut decoded = Project::decode(&encoded, PeerId::new(2)).unwrap();

    let mut file_mut = decoded
        .node_at_path_mut(path!("/foo.txt"))
        .unwrap()
        .unwrap_file()
        .unwrap_text();

    file_mut.edit([TextReplacement {
        deleted_range: 5..5,
        inserted_text: ",".into(),
    }]);

    assert_eq!(file_mut.as_file().contents(), "hello, world");
}
