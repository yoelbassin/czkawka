use crate::help_functions::{
    ColumnsBadExtensions, ColumnsBigFiles, ColumnsBrokenFiles, ColumnsDuplicates, ColumnsEmptyFiles, ColumnsEmptyFolders, ColumnsInvalidSymlinks, ColumnsSameMusic,
    ColumnsSimilarImages, ColumnsSimilarVideos, ColumnsTemporaryFiles, get_list_store,
};
use crate::notebook_enums::NotebookMainEnum;
use crate::notebook_info::{NOTEBOOKS_INFO, NotebookObject};
use glib::translate::ToGlibPtr;
use gtk4::prelude::*;
use gtk4::{Builder, CellRendererText, CellRendererToggle, EventControllerKey, GestureClick, ListStore, Notebook, ScrolledWindow, TreeView, TreeViewColumn};
use gtk4::{CellRendererText, TreeViewColumn};

#[derive(Clone)]
pub struct CommonTreeViews {
    pub subviews: Vec<SubView>,
    pub notebook_main: Notebook,
}
impl CommonTreeViews {
    pub fn get_current(&self) -> SubView {
        let nb_number = self.notebook_main.current_page().expect("Current page not set");
        self.subviews.get(nb_number as usize).expect("Cannot find current notebook tab").clone()
    }
    pub fn get_current_tree_view(&self) -> TreeView {
        let nb_number = self.notebook_main.current_page().expect("Current page not set");
        self.subviews.get(nb_number as usize).expect("Cannot find current notebook tab").tree_view.clone()
    }
    pub fn get_current_model(&self) -> ListStore {
        let nb_number = self.notebook_main.current_page().expect("Current page not set");
        self.subviews
            .get(nb_number as usize)
            .expect("Cannot find current notebook tab")
            .tree_view
            .get_model()
            .clone()
    }
    pub fn setup(&self) {
        for subview in &self.subviews {
            subview.setup();
        }
    }
}

pub trait TreeViewListStoreTrait {
    fn get_model(&self) -> ListStore;
}
impl TreeViewListStoreTrait for TreeView {
    fn get_model(&self) -> ListStore {
        self.model()
            .expect("TreeView has no model")
            .downcast_ref::<ListStore>()
            .expect("TreeView model is not ListStore")
            .clone()
    }
}

#[derive(Clone)]
pub struct SubView {
    pub scrolled_window: ScrolledWindow,
    pub tree_view: TreeView,
    pub gesture_click: GestureClick,
    pub event_controller_key: EventControllerKey,
    pub notebook_object: NotebookObject,
    pub enum_value: NotebookMainEnum,
}

impl SubView {
    pub fn new(builder: &Builder, scrolled_name: &str, enum_value: NotebookMainEnum) -> Self {
        let tree_view: TreeView = TreeView::new();
        let event_controller_key: EventControllerKey = EventControllerKey::new();
        tree_view.add_controller(event_controller_key.clone());
        let gesture_click: GestureClick = GestureClick::new();
        tree_view.add_controller(gesture_click.clone());

        let notebook_object = NOTEBOOKS_INFO[enum_value as usize].clone();

        Self {
            scrolled_window: builder.object(scrolled_name).expect(format!("Cannot find scrolled window {}", scrolled_name).as_str()),
            tree_view,
            gesture_click,
            event_controller_key,
            notebook_object,
            enum_value,
        }
    }

    fn setup(&self) {
        let tree_view = &self.tree_view;
        let model = self.tree_view.get_model();
        self.tree_view.set_vexpand(true);
        match self.enum_value {
            NotebookMainEnum::Duplicate => {
                let columns_colors = (ColumnsDuplicates::Color as i32, ColumnsDuplicates::TextColor as i32);
                let activatable_colors = (ColumnsDuplicates::ActivatableSelectButton as i32, ColumnsDuplicates::Color as i32);

                create_default_selection_button_column(tree_view, ColumnsDuplicates::SelectionButton as i32, model, Some(activatable_colors));

                create_default_column(tree_view, ColumnsDuplicates::Size as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsDuplicates::Name as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsDuplicates::Path as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsDuplicates::Modification as i32, None, Some(columns_colors));
            }
            NotebookMainEnum::EmptyDirectories => {
                create_default_selection_button_column(tree_view, ColumnsEmptyFolders::SelectionButton as i32, model, None);

                create_default_column(tree_view, ColumnsEmptyFolders::Name as i32, Some(None), None);
                create_default_column(tree_view, ColumnsEmptyFolders::Path as i32, Some(None), None);
                create_default_column(
                    tree_view,
                    ColumnsEmptyFolders::Modification as i32,
                    Some(Some(ColumnsEmptyFolders::ModificationAsSecs as i32)),
                    None,
                );
            }
            NotebookMainEnum::BigFiles => {
                create_default_selection_button_column(tree_view, ColumnsBigFiles::SelectionButton as i32, model, None);

                create_default_column(tree_view, ColumnsBigFiles::Size as i32, Some(None), None);
                create_default_column(tree_view, ColumnsBigFiles::Name as i32, Some(None), None);
                create_default_column(tree_view, ColumnsBigFiles::Path as i32, Some(None), None);
                create_default_column(
                    tree_view,
                    ColumnsBigFiles::Modification as i32,
                    Some(Some(ColumnsBigFiles::ModificationAsSecs as i32)),
                    None,
                );
            }
            NotebookMainEnum::EmptyFiles => {
                create_default_selection_button_column(tree_view, ColumnsEmptyFiles::SelectionButton as i32, model, None);

                create_default_column(tree_view, ColumnsEmptyFiles::Name as i32, Some(None), None);
                create_default_column(tree_view, ColumnsEmptyFiles::Path as i32, Some(None), None);
                create_default_column(
                    tree_view,
                    ColumnsEmptyFiles::Modification as i32,
                    Some(Some(ColumnsEmptyFiles::ModificationAsSecs as i32)),
                    None,
                );
            }
            NotebookMainEnum::Temporary => {
                create_default_selection_button_column(tree_view, ColumnsTemporaryFiles::SelectionButton as i32, model, None);

                create_default_column(tree_view, ColumnsTemporaryFiles::Name as i32, Some(None), None);
                create_default_column(tree_view, ColumnsTemporaryFiles::Path as i32, Some(None), None);
                create_default_column(
                    tree_view,
                    ColumnsTemporaryFiles::Modification as i32,
                    Some(Some(ColumnsTemporaryFiles::ModificationAsSecs as i32)),
                    None,
                );
            }
            NotebookMainEnum::SimilarImages => {
                let columns_colors = (ColumnsSimilarImages::Color as i32, ColumnsSimilarImages::TextColor as i32);
                let activatable_colors = (ColumnsSimilarImages::ActivatableSelectButton as i32, ColumnsSimilarImages::Color as i32);

                create_default_selection_button_column(tree_view, ColumnsSimilarImages::SelectionButton as i32, model, Some(activatable_colors));

                create_default_column(tree_view, ColumnsSimilarImages::Similarity as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarImages::Size as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarImages::Dimensions as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarImages::Name as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarImages::Path as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarImages::Modification as i32, None, Some(columns_colors));
            }
            NotebookMainEnum::SimilarVideos => {
                let columns_colors = (ColumnsSimilarVideos::Color as i32, ColumnsSimilarVideos::TextColor as i32);
                let activatable_colors = (ColumnsSimilarVideos::ActivatableSelectButton as i32, ColumnsSimilarVideos::Color as i32);

                create_default_selection_button_column(tree_view, ColumnsSimilarVideos::SelectionButton as i32, model, Some(activatable_colors));

                create_default_column(tree_view, ColumnsSimilarVideos::Size as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarVideos::Name as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarVideos::Path as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSimilarVideos::Modification as i32, None, Some(columns_colors));
            }
            NotebookMainEnum::SameMusic => {
                let columns_colors = (ColumnsSameMusic::Color as i32, ColumnsSameMusic::TextColor as i32);
                let activatable_colors = (ColumnsSameMusic::ActivatableSelectButton as i32, ColumnsSameMusic::Color as i32);

                create_default_selection_button_column(tree_view, ColumnsSameMusic::SelectionButton as i32, model, Some(activatable_colors));

                create_default_column(tree_view, ColumnsSameMusic::Size as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Name as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Title as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Artist as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Year as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Bitrate as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Length as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Genre as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Path as i32, None, Some(columns_colors));
                create_default_column(tree_view, ColumnsSameMusic::Modification as i32, None, Some(columns_colors));
            }
            NotebookMainEnum::Symlinks => {
                create_default_selection_button_column(tree_view, ColumnsInvalidSymlinks::SelectionButton as i32, model, None);

                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsInvalidSymlinks::Name as i32, ColumnSort::Default),
                        (ColumnsInvalidSymlinks::Path as i32, ColumnSort::Default),
                        (ColumnsInvalidSymlinks::DestinationPath as i32, ColumnSort::Default),
                        (ColumnsInvalidSymlinks::TypeOfError as i32, ColumnSort::Default),
                        (ColumnsInvalidSymlinks::Modification as i32, ColumnSort::Custom(ColumnsInvalidSymlinks::ModificationAsSecs as i32)),
                    ],
                    None,
                );
            }
            NotebookMainEnum::BrokenFiles => {
                create_default_selection_button_column(tree_view, ColumnsBrokenFiles::SelectionButton as i32, model, None);

                create_default_column(tree_view, ColumnsBrokenFiles::Name as i32, Some(None), None);
                create_default_column(tree_view, ColumnsBrokenFiles::Path as i32, Some(None), None);
                create_default_column(tree_view, ColumnsBrokenFiles::ErrorType as i32, Some(None), None);
                create_default_column(
                    tree_view,
                    ColumnsBrokenFiles::Modification as i32,
                    Some(Some(ColumnsBrokenFiles::ModificationAsSecs as i32)),
                    None,
                );
            }
            NotebookMainEnum::BadExtensions => {
                create_default_selection_button_column(tree_view, ColumnsBadExtensions::SelectionButton as i32, model, None);

                create_default_column(tree_view, ColumnsBadExtensions::Name as i32, Some(None), None);
                create_default_column(tree_view, ColumnsBadExtensions::Path as i32, Some(None), None);
                create_default_column(tree_view, ColumnsBadExtensions::CurrentExtension as i32, Some(None), None);
                create_default_column(tree_view, ColumnsBadExtensions::ValidExtensions as i32, Some(None), None);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum ColumnSort {
    None,
    Default,
    Custom(i32),
}


pub(crate) fn create_default_selection_button_column(
    tree_view: &TreeView,
    column_id: i32,
    model: ListStore,
    activatable_color_columns: Option<(i32, i32)>,
) -> (CellRendererToggle, TreeViewColumn) {
    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).expect("Failed to get iter from tree_path");
        let mut fixed = model.get::<bool>(&iter, column_id);
        fixed = !fixed;
        model.set_value(&iter, column_id as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", column_id);
    if let Some(activatable_color_columns) = activatable_color_columns {
        column.add_attribute(&renderer, "activatable", activatable_color_columns.0);
        column.add_attribute(&renderer, "cell-background", activatable_color_columns.1);
    }
    tree_view.append_column(&column);
    (renderer, column)
}



fn create_default_columns(
    tree_view: &TreeView,
    columns: &[(i32, ColumnSort)],
    colors_columns_id: Option<(i32, i32)>,
) {
    for (col_id, sort_method) in columns {
        let renderer = CellRendererText::new();
        let column: TreeViewColumn = TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_resizable(true);
        column.set_min_width(50);
        column.add_attribute(&renderer, "text", *col_id);
        match sort_method {
            ColumnSort::None => {},
            ColumnSort::Default => column.set_sort_column_id(*col_id),
            ColumnSort::Custom(val) => column.set_sort_column_id(*val),
        }
        if let Some(colors_columns_id) = colors_columns_id {
            column.add_attribute(&renderer, "background", colors_columns_id.0);
            column.add_attribute(&renderer, "foreground", colors_columns_id.1);
        }
        tree_view.append_column(&column);
    }
}

#[expect(clippy::option_option)]
pub(crate) fn create_default_column(
    tree_view: &TreeView,
    column_id: i32,
    sort_column_id: Option<Option<i32>>,
    colors_columns_id: Option<(i32, i32)>,
) -> (CellRendererText, TreeViewColumn) {
    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", column_id);
    if let Some(sort_column_id) = sort_column_id {
        if let Some(sort_column_id) = sort_column_id {
            column.set_sort_column_id(sort_column_id);
        } else {
            column.set_sort_column_id(column_id);
        }
    }
    if let Some(colors_columns_id) = colors_columns_id {
        column.add_attribute(&renderer, "background", colors_columns_id.0);
        column.add_attribute(&renderer, "foreground", colors_columns_id.1);
    }
    tree_view.append_column(&column);
    (renderer, column)
}
