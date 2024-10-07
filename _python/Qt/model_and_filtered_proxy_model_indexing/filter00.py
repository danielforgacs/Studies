from PyQt6 import QtWidgets
from PyQt6 import QtGui
from PyQt6 import QtCore


class Model(QtGui.QStandardItemModel):
    pass


class ProxyModel(QtCore.QSortFilterProxyModel):
    pass


class Widget(QtWidgets.QTreeView):
    def __init__(self):
        super().__init__()
        model = Model()
        proxy_model = ProxyModel()
        proxy_model.setSourceModel(model)
        self.setModel(proxy_model)


if __name__ == '__main__':
    import sys
    app = QtWidgets.QApplication([])
    win = Widget()
    win.show()
    sys.exit(app.exec())
