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
        for name in list('ABCDE'):
            self.stuffs.append(Stuff(name))


class Main(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        self.setLayout(QtWidgets.QVBoxLayout())

        self.stuffs_view = StuffsView()
        self.delete_button = QtWidgets.QPushButton("Delete")
        self.delete_button.clicked.connect(self.delete)

        self.layout().addWidget(self.stuffs_view)
        self.layout().addWidget(self.delete_button)

    def delete(self):
        print()


class StuffsView(QtWidgets.QListView):
    def __init__(self):
        super().__init__()
        self.setSelectionMode(QtWidgets.QAbstractItemView.SelectionMode.ExtendedSelection)
        self.setModel(QtGui.QStandardItemModel())

    def selectionChanged(self, selected, deselected):
        print(
            "...................................\n"
            f"selected: {selected}\n"
            f"deselected: {deselected}"
        )
        return super().selectionChanged(selected, deselected)


if __name__ == '__main__':
    app = QtWidgets.QApplication([])
    main = Main()
    main.show()
    sys.exit(app.exec())
