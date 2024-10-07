from PyQt6 import QtWidgets
from PyQt6 import QtGui
from PyQt6 import QtCore


class Model(QtGui.QStandardItemModel):
    def reload(self):
        self.appendRow(QtGui.QStandardItem('a'))
        self.appendRow(QtGui.QStandardItem('ab'))
        self.appendRow(QtGui.QStandardItem('abc'))
        self.appendRow(QtGui.QStandardItem('bcd'))
        self.appendRow(QtGui.QStandardItem('cde'))
        self.appendRow(QtGui.QStandardItem('de'))
        self.appendRow(QtGui.QStandardItem('e'))


class ProxyModel(QtCore.QSortFilterProxyModel):
    def set_filter(self, text):
        self.setFilterFixedString(text)


class Widget(QtWidgets.QTreeView):
    def __init__(self):
        super().__init__()
        model = Model()
        proxy_model = ProxyModel()
        proxy_model.setSourceModel(model)
        self.setModel(proxy_model)
        self.selectionModel().selectionChanged.connect(self.on_selection_changed)
        model.reload()
        proxy_model.set_filter('b')


    def on_selection_changed(self, selected, deselected):
        print(selected, deselected)


if __name__ == '__main__':
    import sys
    app = QtWidgets.QApplication([])
    win = Widget()
    win.show()
    sys.exit(app.exec())
