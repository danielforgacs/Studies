from PyQt6 import QtWidgets
from PyQt6 import QtGui
from PyQt6 import QtCore


class Model(QtGui.QStandardItemModel):
    def reload(self):
        self.appendRow(QtGui.QStandardItem('a'))
        self.appendRow(QtGui.QStandardItem('b'))
        self.appendRow(QtGui.QStandardItem('c'))


class ProxyModel(QtCore.QSortFilterProxyModel):
    pass


class SelectionModel(QtCore.QItemSelectionModel):
    pass


class Widget(QtWidgets.QTreeView):
    def __init__(self):
        super().__init__()
        model = Model()
        proxy_model = ProxyModel()
        sel_model = SelectionModel()
        proxy_model.setSourceModel(model)
        self.setModel(proxy_model)
        self.setSelectionModel(sel_model)
        sel_model.selectionChanged.connect(self.on_selection_changed)
        model.reload()


    def on_selection_changed(self, *args, **kwargs):
        print('l')
        print(args)
        print(kwargs)



if __name__ == '__main__':
    import sys
    app = QtWidgets.QApplication([])
    win = Widget()
    win.show()
    sys.exit(app.exec())
