import sys
from PyQt6 import QtCore
from PyQt6 import QtGui
from PyQt6 import QtWidgets


class Stuff:
    def __init__(self, name: str):
        self.name = name

    def display(self) -> str:
        return f"stuff: {self.name}"


class Data:
    def __init__(self):
        self.stuffs = []
        for name in list(
            "D"
            "G"
            "B"
            "C"
            "A"
            "E"
            "F"
        ):
            self.stuffs.append(Stuff(name))


class Main(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        self.setLayout(QtWidgets.QVBoxLayout())

        self.stuffs_view = StuffsView()
        self.delete_button = QtWidgets.QPushButton("Delete")
        self.delete_button.clicked.connect(self.stuffs_view.delete_selected)

        self.layout().addWidget(self.stuffs_view)
        self.layout().addWidget(self.delete_button)

        self.data = Data()

        for stuff in self.data.stuffs:
            self.stuffs_view.append_stuff(stuff)


class StuffsView(QtWidgets.QListView):
    def __init__(self):
        super().__init__()
        self.setSelectionMode(QtWidgets.QAbstractItemView.SelectionMode.ExtendedSelection)
        proxy = QtCore.QSortFilterProxyModel()
        proxy.setSourceModel(QtGui.QStandardItemModel())
        self.setModel(proxy)
        self.model().sort(0)

    def append_stuff(self, stuff: Stuff):
        item = QtGui.QStandardItem(stuff.display())
        item.setData(QtCore.Qt.ItemDataRole.UserRole)
        self.model().sourceModel().appendRow(item)

    def delete_selected(self):
        proxy = self.model()
        model = proxy.sourceModel()
        print('..................................')
        for index in self.selectedIndexes():
            proxy_index = index.row()
            model_index = proxy.mapToSource(index).row()
            print(f"proxy: {proxy_index}, model: {model_index}")


if __name__ == '__main__':
    app = QtWidgets.QApplication([])
    main = Main()
    main.show()
    sys.exit(app.exec())
