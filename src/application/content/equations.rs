use crate::parser::parser::MathTree;
use crate::parser::parser_nodes::NullNode;

use std::rc::Rc;
use std::cell::{LazyCell, RefCell};
use std::sync::Mutex;
use std::boxed::Box;

use adw::prelude::*;
use adw::{EntryRow, ActionRow, ButtonContent};
use adw::gtk::{ListBox, ScrolledWindow, SelectionMode, Image, Button, Orientation};
use adw::gtk::Box as VertBox;
use adw::gdk::{Display};

trait RemoveValue {
    fn remove_value(&mut self, value: i32);
}

impl RemoveValue for Vec<i32> {
    fn remove_value(&mut self, value: i32) {
        self.retain(|x| *x != value);
    }
}

struct MathRow {
    tree: MathTree,
    input: String,
    list_box: ListBox,
    math_box: EntryRow,
    output_box: ActionRow,
    id: i32,
}

impl MathRow {
    pub fn new() -> Self {
        let row = Self {
            tree: MathTree::new(Box::new(NullNode::new())),
            input: "".to_string(),
            list_box: ListBox::builder()
                .margin_top(8)
                .margin_end(32)
                .margin_bottom(8)
                .margin_start(32)
                .selection_mode(SelectionMode::None)
                .css_classes(vec![String::from("boxed-list")])
                .build(),
            math_box: EntryRow::new(),
            output_box: ActionRow::builder()
                .activatable(true)
                .title(format!("= Null"))
                .build(),
            id: -1,
        };

        return row;
    }

    fn inserted_into_list(this: &Rc<RefCell<Self>>, id: i32) {
        {
            let mut row = this.borrow_mut();
            row.id = id;
        }

        Self::create_math_box(&this.clone());
        Self::create_output_box(&this.clone());

        let row = this.borrow();
        row.list_box.append(&row.math_box.clone());
        row.list_box.append(&row.output_box.clone());
    }

    fn create_output_box(this: &Rc<RefCell<Self>>) {
        let row = this.borrow();

        let copy_symbol = Image::from_icon_name("edit-copy-symbolic");

        let remove_button_content = ButtonContent::builder()
            .icon_name("user-trash-symbolic")
            .build();
        let remove_button = Button::builder()
            .child(&remove_button_content)
            .build();
        let remove_button_style = remove_button.style_context();
        remove_button_style.add_class("destructive-action");

        let id = row.id;
        let list_box = row.list_box.clone();
        remove_button.connect_clicked(move |_| {
            remove_math_row(id, list_box.clone());
        });

        let end_buttons = VertBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .margin_top(8)
            .margin_bottom(8)
            .build();
        end_buttons.append(&copy_symbol);
        end_buttons.append(&remove_button);

        row.output_box.add_suffix(&end_buttons);

        let output_box_clone = row.output_box.clone();
        row.output_box.connect_activated(move |_| {
            let display = Display::default().expect("no display found");
            let clipboard = display.clipboard();
            clipboard.set_text(output_box_clone.clone().title().to_string().strip_prefix("= ").unwrap());
        }); 
    }

    fn create_math_box(this: &Rc<RefCell<Self>>) {
        let row = this.borrow();
        let copy_button_content = ButtonContent::builder()
            .icon_name("edit-copy-symbolic")
            .build();
        let copy_button = Button::builder()
            .child(&copy_button_content)
            .build();

        let math_box_clone = row.math_box.clone();

        copy_button.connect_clicked(move |_| {
            let display = Display::default().expect("no display found");
            let clipboard = display.clipboard();
            clipboard.set_text(&math_box_clone.clone().text().to_string());
        });

        let end_buttons = VertBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .margin_top(8)
            .margin_bottom(8)
            .build();
        end_buttons.append(&copy_button);

        row.math_box.add_suffix(&end_buttons);
        
        let math_box_clone = row.math_box.clone();
        
        drop(row);
        let this_clone = this.clone(); 
        math_box_clone.connect_changed(move |_| {
            Self::activated_input_box(&this_clone, );
        });
    }

    fn activated_input_box(this: &Rc<RefCell<Self>>) {
        let mut row = this.borrow_mut();
        test_should_add_row(row.id);
        let input = row.math_box.text().to_string();
        row.input = input.clone();
        row.tree.set_input(input.clone());
        let result = row.tree.get_result(); 
        if !result.is_none() {
            row.output_box.set_title(&format!("= {}", result.unwrap()));
        } else {
            row.output_box.set_title(&format!("= Null"));
        }
    }

    fn get_list_box(&self) -> ListBox {
        self.list_box.clone()
    }
}

static ACTIVE_IDS: Mutex<Vec<i32>> = Mutex::new(Vec::<i32>::new());

thread_local! {
    static VERTICAL_BOX: LazyCell<VertBox> = LazyCell::new(|| VertBox::new(Orientation::Vertical, 0));
    static SCROLLED_WINDOW: LazyCell<ScrolledWindow> = LazyCell::new(|| ScrolledWindow::new());
}


fn insert_math_row(row_cell: &Rc<RefCell<MathRow>>) {
    let mut new_id: i32 = 0;
    {
        let row = row_cell.borrow();
        let mut active_ids = ACTIVE_IDS.lock().unwrap(); 
        if active_ids.len() > 0 {
            new_id = active_ids.get(active_ids.len() - 1).unwrap() + 1;
        }

        active_ids.push(new_id);
        drop(active_ids);

        VERTICAL_BOX.with(|v| {
            v.append(&row.get_list_box());
        });
    }

    MathRow::inserted_into_list(&row_cell.clone(), new_id);
}

fn test_should_add_row(id: i32) {
    let active_ids = ACTIVE_IDS.lock().unwrap();
    let latest_id = *active_ids.get(active_ids.len() - 1).unwrap();
    drop(active_ids);

    if latest_id == id {
        create_new_math_row();
    }
}

fn create_new_math_row() {
    let row: Rc<RefCell<MathRow>> = Rc::new(RefCell::new(MathRow::new()));
    insert_math_row(&row);
}

fn remove_math_row(id: i32, list_box: ListBox) {
    let mut active_ids = ACTIVE_IDS.lock().unwrap();
    let mut is_last = false;

    if *active_ids.get(active_ids.len() - 1).unwrap() == id {
        is_last = true;
    }
    active_ids.remove_value(id);
    drop(active_ids);
    VERTICAL_BOX.with(|v| {
        v.remove(&list_box);
    });

    if is_last {
        create_new_math_row();
    }
}
    
pub fn get_scrolled_list() -> ScrolledWindow {
    SCROLLED_WINDOW.with(|s| (**s).clone())
}

pub fn create_equations() {
    create_new_math_row();
    SCROLLED_WINDOW.with(|s| {
        s.set_child(Some(&VERTICAL_BOX.with(|v| (**v).clone())));
    });
}
