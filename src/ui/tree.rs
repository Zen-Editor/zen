use std::path::PathBuf;

#[derive(Clone)]
pub struct FileTreeNode {
    pub path: PathBuf,
    pub is_directory: bool,
    pub children: Vec<FileTreeNode>,
    pub expanded: bool,
}

pub struct FileExplorer {
    pub root: Option<FileTreeNode>,
    pub selected_file: Option<PathBuf>,
    pub pending_file_load: Option<PathBuf>,
}

impl Default for FileExplorer {
    fn default() -> Self {
        Self {
            root: None,
            selected_file: None,
            pending_file_load: None,
        }
    }
}

impl FileExplorer {
    pub fn open_project(&mut self, path: PathBuf) {
        self.root = Some(self.build_file_tree(&path, false));
    }

    fn build_file_tree(&self, dir: &PathBuf, expanded: bool) -> FileTreeNode {
        let mut children = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut entries: Vec<_> = entries.collect();
            entries.sort_by(|a, b| {
                let a = a.as_ref().unwrap();
                let b = b.as_ref().unwrap();
                let a_is_dir = a.path().is_dir();
                let b_is_dir = b.path().is_dir();

                match (a_is_dir, b_is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.file_name().cmp(&b.file_name()),
                }
            });

            for entry in entries.into_iter().flatten() {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();

                if file_name.starts_with('.') || file_name == "target" {
                    continue;
                }

                if path.is_dir() {
                    children.push(self.build_file_tree(&path, false));
                } else {
                    children.push(FileTreeNode {
                        path,
                        is_directory: false,
                        children: Vec::new(),
                        expanded: false,
                    });
                }
            }
        }

        FileTreeNode {
            path: dir.clone(),
            is_directory: true,
            children,
            expanded,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Explorer");
        ui.separator();

        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if let Some(ref mut tree) = self.root.clone() {
                    self.render_node(ui, tree, 0);
                    self.root = Some(tree.clone());
                }
            });
    }

    fn render_node(&mut self, ui: &mut egui::Ui, node: &mut FileTreeNode, depth: usize) {
        let indent = depth as f32 * 15.0;

        ui.horizontal(|ui| {
            ui.add_space(indent);

            if node.is_directory {
                let icon = if node.expanded { "📂" } else { "📁" };
                let name = node.path.file_name().unwrap().to_string_lossy();
                if ui.selectable_label(false, format!("{} {}", icon, name)).clicked() {
                    node.expanded = !node.expanded;
                }
            } else {
                let icon = "📄";
                let name = node.path.file_name().unwrap().to_string_lossy();
                if ui.selectable_label(
                    self.selected_file.as_ref() == Some(&node.path),
                    format!("{} {}", icon, name)
                ).clicked() {
                    self.pending_file_load = Some(node.path.clone());
                    self.selected_file = Some(node.path.clone());
                }
            }
        });

        if node.is_directory && node.expanded {
            for child in &mut node.children {
                self.render_node(ui, child, depth + 1);
            }
        }
    }

    pub fn take_pending_file(&mut self) -> Option<PathBuf> {
        self.pending_file_load.take()
    }
}