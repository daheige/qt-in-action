#![windows_subsystem = "windows"]

use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs};
use qt_widgets::{
    QAction, QApplication, QLineEdit, QMenu, QMessageBox, QPushButton, QTableWidget,
    QTableWidgetItem, QVBoxLayout, QWidget, SlotOfQPoint, SlotOfQTableWidgetItemQTableWidgetItem,
};
use std::rc::Rc;

// 定义form组件信息
struct Form {
    widget: QBox<QWidget>,
    line_edit: QBox<QLineEdit>,
    button: QBox<QPushButton>,
    table: QBox<QTableWidget>,
}

impl StaticUpcast<QObject> for Form {
    // 将form转换为QObject
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Form {
    // 创建form对象，并用rc进行包裹起来，成为一个引用计数的form
    fn new() -> Rc<Self> {
        unsafe {
            let widget = QWidget::new_0a();
            let layout = QVBoxLayout::new_1a(&widget);
            let line_edit = QLineEdit::new();
            layout.add_widget(&line_edit);

            // 创建按钮
            let button = QPushButton::from_q_string(&qs("Start"));
            button.set_enabled(false);
            layout.add_widget(&button); // 添加到layout

            // 创建table
            let table = QTableWidget::new_0a();
            let policy = ContextMenuPolicy::CustomContextMenu;
            table.set_context_menu_policy(policy);
            table.set_row_count(2); // 设置行的数量
            table.set_column_count(1); // 设置列的数量

            let item1 = QTableWidgetItem::new().into_ptr();
            item1.set_text(&qs("item1")); // 设置第1行第1列文本内容
            table.set_item(0, 0, item1);

            let item2 = QTableWidgetItem::new().into_ptr();
            item2.set_text(&qs("item2")); // 设置第2行第2列文本内容
            table.set_item(1, 0, item2);

            layout.insert_widget_2a(0, &table); // 插入表格

            widget.show(); // 展示组件

            let f = Rc::new(Self {
                widget,
                button,
                line_edit,
                table,
            });

            f.init();
            f
        }
    }

    // 初始化操作
    unsafe fn init(self: &Rc<Self>) {
        self.button
            .clicked()
            .connect(&self.slot_on_button_clicked());
        self.line_edit
            .text_edited()
            .connect(&self.slot_on_line_edit_text_edited());
        self.table
            .current_item_changed()
            .connect(&self.slot_on_table_current_item_changed());
        self.table
            .custom_context_menu_requested()
            .connect(&self.slot_on_table_context_menu_requested());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_line_edit_text_edited(self: &Rc<Self>) {
        self.button.set_enabled(!self.line_edit.text().is_empty());
    }

    // 按钮点击操作
    #[slot(SlotNoArgs)]
    unsafe fn on_button_clicked(self: &Rc<Self>) {
        let text = self.line_edit.text();
        QMessageBox::information_q_widget2_q_string(
            &self.widget,
            &qs("Example"),
            &qs("Text: \"%1\". Congratulations!").arg_q_string(&text),
        );
    }

    // table表格中内容改变事件
    #[slot(SlotOfQTableWidgetItemQTableWidgetItem)]
    unsafe fn on_table_current_item_changed(
        self: &Rc<Self>,
        current: Ptr<QTableWidgetItem>,
        previous: Ptr<QTableWidgetItem>,
    ) {
        if !previous.is_null() {
            let font = previous.font();
            font.set_bold(false);
            previous.set_font(&font);
        }
        if !current.is_null() {
            let font = current.font();
            font.set_bold(true);
            current.set_font(&font);
        }
    }

    #[slot(SlotOfQPoint)]
    unsafe fn on_table_context_menu_requested(self: &Rc<Self>, pos: Ref<QPoint>) {
        let global_pos = self.table.viewport().map_to_global(pos);
        let menu = QMenu::new();
        menu.add_action_q_string(&qs("Fake action 1"));
        menu.add_action_q_string(&qs("Fake action 2"));

        let action3 = QAction::from_q_string(&qs("Real action"));
        menu.add_action(&action3);

        let action = menu.exec_1a_mut(&global_pos);
        let message = if action.is_null() {
            "No action selected!".to_string()
        } else if action.as_raw_ptr() == action3.as_raw_ptr() {
            "Real action selected!".to_string()
        } else {
            format!("Fake action selected ({})", action.text().to_std_string())
        };
        QMessageBox::information_q_widget2_q_string(&self.widget, &qs("Example"), &qs(message));
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}
