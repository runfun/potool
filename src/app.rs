use std::{cell::RefCell, path::PathBuf, rc::Rc};

use fltk::{enums::Event, prelude::*, *};
use frame::Frame;
use group::FlexType;

use crate::proc::build_po;

pub struct PotoolApp {
    state: Rc<RefCell<AppState>>,
}

impl PotoolApp {
    pub fn new() -> Self {
        PotoolApp {
            state: Rc::new(RefCell::new(AppState {
                tasks: vec![],
                case_sensitive: false,
            })),
        }
    }

    pub fn run(self) {
        let app = app::App::default();
        let mut wind = window::Window::default().with_size(1280, 720);

        let mut flex = group::Flex::default()
            .with_size(1200, 600)
            .with_type(FlexType::Column)
            .center_of_parent();
        let pot =
            FilePickerWidget::file_picker("pot file", dialog::FileDialogType::BrowseFile, "*.pot");
        let out =
            FilePickerWidget::file_picker("output dir", dialog::FileDialogType::BrowseDir, "");
        let mut cb_case = button::CheckButton::default().with_label("Case sensitive");
        flex.fixed(&pot.flex, 32);
        flex.fixed(&out.flex, 32);
        flex.fixed(&cb_case, 32);
        flex.end();

        let mut pack = group::Pack::default()
            .with_size(300, 32)
            .with_type(group::PackType::Horizontal)
            .below_of(&flex, 12);
        let mut btn_add = button::Button::new(10, 10, 100, 32, "Add Task");
        let mut btn_start = button::Button::new(10, 10, 100, 32, "Start");
        pack.auto_layout();
        pack.end();

        wind.end();
        wind.show();

        let mut inner_flex = flex.clone();
        let inner_state = Rc::clone(&self.state);
        btn_add.set_callback(move |_| {
            let task = TaskWidget::new();
            inner_state.borrow_mut().tasks.push(task);
            let s = inner_state.borrow_mut();
            let f = &s.tasks.last().unwrap().flex;
            inner_flex.add(f);
            inner_flex.fixed(f, 32);
            inner_flex.redraw();
        });

        let state = Rc::clone(&self.state);
        btn_start.set_callback(move |_| {
            let pot_file = PathBuf::from(pot.buf.text());
            let datas = state.borrow_mut().datas();
            let src_files = datas.iter().map(|d| d.0.clone()).collect::<Vec<_>>();
            let out_files = datas
                .iter()
                .map(|d| PathBuf::from(format!("{}\\{}.po", out.buf.text(), d.1.to_str().unwrap())))
                .collect::<Vec<_>>();

            build_po(pot_file, src_files, out_files, "", state.borrow().case_sensitive);
        });

        let state = self.state;
        cb_case.set_callback(move |cb| {
            if cb.is_checked() {
                state.borrow_mut().case_sensitive = true;
            } else {
                state.borrow_mut().case_sensitive = false;
            }
        });

        app.run().unwrap();
    }
}

struct AppState {
    tasks: Vec<TaskWidget>,
    case_sensitive: bool,
}

impl AppState {
    fn datas(&self) -> Vec<(PathBuf, PathBuf)> {
        self.tasks
            .iter()
            .map(|t| (PathBuf::from(t.buf.text()), PathBuf::from(t.input.value())))
            .collect()
    }
}

struct FilePickerWidget {
    flex: group::Flex,
    buf: text::TextBuffer,
}

impl FilePickerWidget {
    fn file_picker(title: &str, op: dialog::FileDialogType, filter: &'static str) -> Self {
        let mut h_flex = group::Flex::default().with_size(300, 32);
        let label = Frame::default().with_label(title);

        let disp = DragTextDisplay::new();

        let mut btn_clear = button::Button::default().with_label("x");
        let mut btn_select = button::Button::default().with_label("...");
        h_flex.fixed(&label, 64);
        h_flex.fixed(&btn_clear, 32);
        h_flex.fixed(&btn_select, 32);
        h_flex.end();

        let mut inner_buf = disp.buf.clone();
        btn_clear.set_callback(move |_| inner_buf.set_text(""));

        let mut inner_buf = disp.buf.clone();
        btn_select.set_callback(move |_| {
            let mut dialog = dialog::FileDialog::new(op);
            if op == dialog::FileDialogType::BrowseFile {
                dialog.set_filter(filter);
            }
            dialog.show();
            let path = dialog.filename();

            if path.is_file() {
                inner_buf.set_text(path.to_str().unwrap());
            } else if path.is_dir() {
                inner_buf.set_text(path.to_str().unwrap());
            }
        });

        return FilePickerWidget {
            flex: h_flex,
            buf: disp.buf.clone(),
        };
    }
}

struct TaskWidget {
    flex: group::Flex,
    buf: text::TextBuffer,
    input: input::Input,
}

impl TaskWidget {
    fn new() -> Self {
        let mut flex = group::Flex::default().with_size(300, 32);
        let task =
            FilePickerWidget::file_picker("task", dialog::FileDialogType::BrowseFile, "*.ini");
        let gap = Frame::default().with_size(10, 32);
        let input = input::Input::default().with_size(200, 32);
        let frame = Frame::default().with_label(".po");

        flex.fixed(&frame, 32);
        flex.fixed(&gap, 32);
        flex.fixed(&input, 128);
        flex.end();

        return TaskWidget {
            flex,
            buf: task.buf,
            input,
        };
    }
}

struct DragTextDisplay {
    buf: text::TextBuffer,
}

impl DragTextDisplay {
    fn new() -> Self {
        let buf = text::TextBuffer::default();
        let mut disp = text::TextDisplay::default_fill();

        disp.set_buffer(buf.clone());
        disp.handle({
            let mut dnd = false;
            let mut released = false;
            let mut buf = buf.clone();
            move |_, ev| match ev {
                Event::DndEnter => {
                    dnd = true;
                    true
                }
                Event::DndDrag => true,
                Event::DndRelease => {
                    released = true;
                    true
                }
                Event::Paste => {
                    if dnd && released {
                        let path = app::event_text();
                        buf.append(&path);
                        dnd = false;
                        released = false;
                        true
                    } else {
                        false
                    }
                }
                Event::DndLeave => {
                    dnd = false;
                    released = false;
                    true
                }
                _ => false,
            }
        });

        return DragTextDisplay { buf };
    }
}
