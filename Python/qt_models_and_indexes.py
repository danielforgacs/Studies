import sys
from PyQt6 import QtCore
from PyQt6 import QtGui
from PyQt6 import QtWidgets


class Stuff:
    def __init__(self, name: str):
        self.name = name

    def display(self) -> str:
        return f"stuff: {self.name}"


class Main(QtWidgets.QListView):
    def __init__(self):
        super().__init__()
        self.setSelectionMode(QtWidgets.QAbstractItemView.SelectionMode.ExtendedSelection)
        self.setModel(QtGui.QStandardItemModel())

        for name in list('ABCDE'):
            instance = Stuff(name)
            item = QtGui.QStandardItem(instance.display())
            item.setData(instance, QtCore.Qt.ItemDataRole.UserRole)
            self.model().appendRow(item)

    def selectionChanged(self, selected, deselected):
        print(
            "...................................\n"
            f"selected: {selected}\n"
            f"deselected: {deselected}"
        )
        return super().selectionChanged(selected, deselected)


if __name__ == '__main__':
    app = QtWidgets.QApplication([])
    # app = QtGui.QGuiApplication([])
    main = Main()
    main.show()
    sys.exit(app.exec())
