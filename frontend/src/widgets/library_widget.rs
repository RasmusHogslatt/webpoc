use crate::{
    app_states::{FilterState, LibraryViewState, SortState},
    singletons::Singletons,
};
use egui::{ComboBox, Response, Ui, Widget};
use shared::{custom_traits::*, holders, library::Library, tools::tool::Tool};
use shared::{holders::holder::Holder, User};

pub struct LibraryWidget<'a> {
    user: &'a mut User,
    singletons: &'a mut Singletons,
}

impl<'a> LibraryWidget<'a> {
    pub fn new(user: &'a mut User, singletons: &'a mut Singletons) -> Self {
        Self { user, singletons }
    }

    fn sort_and_filter_items<T>(&self, items: &mut Vec<T>)
    where
        T: GetRotatingToolCategory + GetTurningToolCategory + GetDiameter + GetDegree,
    {
        // Filter the items
        items.retain(|item| {
            match (
                self.singletons.library_view_state,
                self.singletons.filter_state,
            ) {
                (LibraryViewState::Tool, FilterState::RotatingToolCategory) => {
                    matches!(item.get_rotating_tool_category(), Some(_))
                }
                (LibraryViewState::Tool, FilterState::TurningToolCategory) => {
                    matches!(item.get_turning_tool_category(), Some(_))
                }
                (LibraryViewState::Holder, FilterState::RotatingHolderCategory) => {
                    matches!(item.get_rotating_tool_category(), Some(_))
                }
                (LibraryViewState::Holder, FilterState::TurningHolderCategory) => {
                    matches!(item.get_turning_tool_category(), Some(_))
                }
                _ => false,
            }
        });

        // Sort the filtered items
        match self.singletons.sort_state {
            SortState::Index => {} // Already sorted by index
            SortState::Diameter => items.sort_by(|a, b| {
                a.get_diameter()
                    .partial_cmp(&b.get_diameter())
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            SortState::Degree => items.sort_by(|a, b| {
                a.get_degree()
                    .partial_cmp(&b.get_degree())
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
        }
    }
}

impl<'a> Widget for LibraryWidget<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let view_state = &mut self.singletons.library_view_state;
        let sort_state = &mut self.singletons.sort_state;
        let filter_state = &mut self.singletons.filter_state;
        let library = &mut self.user.user_data.library;

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.radio_value(view_state, LibraryViewState::Tool, "Tool");
                ui.radio_value(view_state, LibraryViewState::Holder, "Holder");
            });

            display_sort_options(ui, sort_state);
            display_main_filter_options(ui, filter_state, &view_state);

            // Do some filtering and sorting to generate a new vector that should be displayed
            self.singletons.libaray_display_tools = filter_tools(library, filter_state);
            self.singletons.library_display_holders = filter_holders(library, filter_state);
        });

        let mut delete_tool_uuid: Option<String> = None;
        let mut delete_holder_uuid: Option<String> = None;
        let mut add_holder_copy_uuid: Option<String> = None;

        match view_state {
            LibraryViewState::Tool => {
                egui::Grid::new("tools").num_columns(1).show(ui, |ui| {
                    for tool in self.singletons.libaray_display_tools.iter() {
                        tool.display(ui);
                        if ui.button("Delete").clicked() {
                            delete_tool_uuid = Some(tool.get_uuid());
                        }
                        ui.end_row();
                    }
                });
            }
            LibraryViewState::Holder => {
                egui::Grid::new("holders").num_columns(1).show(ui, |ui| {
                    for holder in self.singletons.library_display_holders.iter_mut() {
                        holder.display(ui);

                        if ui.button("Delete").clicked() {
                            delete_holder_uuid = Some(holder.get_uuid());
                        }

                        if ui.button("Add Copy").clicked() {
                            add_holder_copy_uuid = Some(holder.get_uuid());
                        }

                        ui.end_row();
                    }
                });
            }
        }

        if let Some(uuid) = delete_tool_uuid {
            if let Some(tool) = self
                .user
                .user_data
                .library
                .tools
                .iter_mut()
                .find(|x| x.get_uuid() == uuid)
            {
                self.user
                    .user_data
                    .library
                    .tools
                    .retain(|x| x.get_uuid() != uuid);
            }
        }

        if let Some(uuid) = add_holder_copy_uuid {
            if let Some(holder) = self
                .user
                .user_data
                .library
                .holders
                .iter_mut()
                .find(|item| item.get_uuid() == uuid)
            {
                holder.add_copy();
            }
        }

        if let Some(uuid) = delete_holder_uuid {
            if let Some(holder) = self
                .user
                .user_data
                .library
                .holders
                .iter_mut()
                .find(|item| item.get_uuid() == uuid)
            {
                if holder.delete_holder() {
                    self.user
                        .user_data
                        .library
                        .holders
                        .retain(|item| item.get_uuid() != uuid);
                }
            }
        }

        ui.centered_and_justified(|ui| ui.label("Library Widget"))
            .response
    }
}

fn display_main_filter_options(
    ui: &mut Ui,
    filter_state: &mut FilterState,
    view_state: &LibraryViewState,
) {
    ComboBox::from_label("Filter by")
        .selected_text(format!("{:?}", filter_state))
        .show_ui(ui, |ui| match view_state {
            LibraryViewState::Tool => {
                ui.selectable_value(filter_state, FilterState::ShowAll, "Show All");
                ui.selectable_value(
                    filter_state,
                    FilterState::RotatingToolCategory,
                    "Rotating Tools",
                );
                ui.selectable_value(
                    filter_state,
                    FilterState::TurningToolCategory,
                    "Turning Tools",
                );
            }
            LibraryViewState::Holder => {
                ui.selectable_value(filter_state, FilterState::ShowAll, "Show All");
                ui.selectable_value(
                    filter_state,
                    FilterState::RotatingHolderCategory,
                    "Rotating Holders",
                );
                ui.selectable_value(
                    filter_state,
                    FilterState::TurningHolderCategory,
                    "Turning Holders",
                );
            }
        });
}

fn display_sort_options(ui: &mut Ui, sort_state: &mut SortState) {
    ComboBox::from_label("Sort by")
        .selected_text(format!("{:?}", sort_state))
        .show_ui(ui, |ui| {
            ui.selectable_value(sort_state, SortState::Index, "Index");
            ui.selectable_value(sort_state, SortState::Diameter, "Diameter");
            ui.selectable_value(sort_state, SortState::Degree, "Degree");
        });
}

fn filter_holders(library: &Library, filter_state: &FilterState) -> Vec<Holder> {
    match filter_state {
        FilterState::ShowAll => library.holders.clone(),
        FilterState::RotatingHolderCategory => library
            .holders
            .iter()
            .filter(|holder| holder.is_rotating())
            .cloned()
            .collect(),

        FilterState::TurningHolderCategory => library
            .holders
            .iter()
            .filter(|holder| holder.is_turning())
            .cloned()
            .collect(),
        FilterState::RotatingToolCategory => {
            vec![]
        }
        FilterState::TurningToolCategory => {
            vec![]
        }
    }
}
fn filter_tools(library: &Library, filter_state: &FilterState) -> Vec<Tool> {
    match filter_state {
        FilterState::RotatingToolCategory => library
            .tools
            .iter()
            .filter(|tool| tool.is_rotating())
            .cloned()
            .collect(),
        FilterState::RotatingHolderCategory => {
            vec![]
        }
        FilterState::TurningToolCategory => library
            .tools
            .iter()
            .filter(|tool| tool.is_turning())
            .cloned()
            .collect(),
        FilterState::TurningHolderCategory => vec![],
        FilterState::ShowAll => library.tools.clone(),
    }
}

// fn filter_tools(library: &Library, filter_state: &FilterState) -> Vec<Tool> {}

// Filter should use trait is_holder/turning to choose what to filter by and
// use filter_state to choose what to sort by and use trait to check if holder/tool is that type
