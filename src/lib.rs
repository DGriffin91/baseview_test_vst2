//! Barebones baseview vst2 plugin that logs events to ~/tmp/BaseviewTest.log

#[macro_use]
extern crate vst;

use baseview::{
    Size, Event, Parent, Window, WindowHandler, WindowOpenOptions,
    WindowScalePolicy
};
use raw_window_handle::RawWindowHandle;
use vst::plugin::{Info, Plugin};
use vst::editor::Editor;


const PLUGIN_NAME: &str = "BaseviewTest";

const WINDOW_WIDTH: usize = 500;
const WINDOW_HEIGHT: usize = 500;


#[derive(Default)]
struct TestWindowHandler;


impl WindowHandler for TestWindowHandler {
    fn on_event(&mut self, _: &mut Window, event: Event) {
        ::log::info!("TestWindowHandler received event: {:?}", event)
    }

    fn on_frame(&mut self) {
        
    }
}


#[derive(Default)]
struct TestPluginEditor {
    is_open: bool,
}


impl Editor for TestPluginEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        if self.is_open {
            return false;
        }

        let parent = raw_window_handle_from_parent(parent);

        let options = WindowOpenOptions {
            title: PLUGIN_NAME.to_string(),
            size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
            parent: Parent::WithParent(parent),
        };

        Window::open(options, |_|{
            TestWindowHandler::default()
        });

        true
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
    }
}


struct TestPlugin {
    editor: Option<TestPluginEditor>,
}


impl Default for TestPlugin {
    fn default() -> Self {
        Self {
            editor: Some(TestPluginEditor::default()),
        }
    }
}


impl Plugin for TestPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            unique_id: 53435,

            ..Default::default()
        }
    }

    fn init(&mut self) {
        let log_folder = ::dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

        let log_file = ::std::fs::File::create(
            log_folder.join(format!("{}.log", PLUGIN_NAME))
        ).unwrap();

        let log_config = ::simplelog::ConfigBuilder::new()
            .set_time_to_local(true)
            .build();

        let _ = ::simplelog::WriteLogger::init(
            simplelog::LevelFilter::Info,
            log_config,
            log_file
        );  

        ::log_panics::init();

        ::log::info!("init");
    }   

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take(){
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }
}


#[cfg(target_os = "macos")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::macos::MacOSHandle;

    RawWindowHandle::MacOS(MacOSHandle {
        ns_view: parent,
        ..MacOSHandle::empty()
    })
}


#[cfg(target_os = "windows")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::windows::WindowsHandle;

    RawWindowHandle::Windows(WindowsHandle {
        hwnd: parent,
        ..WindowsHandle::empty()
    })
}


#[cfg(target_os = "linux")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::unix::XcbHandle;

    RawWindowHandle::Xcb(XcbHandle {
        window: parent as u32,
        ..XcbHandle::empty()
    })
}


plugin_main!(TestPlugin);
