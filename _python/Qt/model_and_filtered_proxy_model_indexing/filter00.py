from PyQt6 import QtWidgets
from PyQt6 import QtGui
from PyQt6 import QtCore

data = (
    'a',
    'b',
    'ab',
    'bc',
    'abc',
    'bcde',
    'abcde',
)


class Dummy:
    def __init__(self, num):
        self.num = num * 10


class Model(QtGui.QStandardItemModel):
    def reload(self):
        for idx, item in enumerate(data):
            item = QtGui.QStandardItem(f'{item} - {idx}')
            item.setData(Dummy(idx), 3)
            item2 = QtGui.QStandardItem(str(idx))
            self.appendRow((item, item2))


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
        proxy_model.set_filter('a')
        self.setSortingEnabled(True)


    def on_selection_changed(self, selected, deselected):
        print('-----------------')
        item = selected.indexes()[0]
        print(item.data(3).num)
        print(selected, deselected)
        print([idx.row() for idx in selected.indexes()])
        print([idx.row() for idx in self.selectedIndexes()])
        print(self.model())
        # print(self.model().mapSelectionToSource)
        source_model_index = self.model().mapToSource(self.selectedIndexes()[0])
        print(source_model_index)
        data = self.model().sourceModel().itemFromIndex(source_model_index)
        print(data)


if __name__ == '__main__':
    import sys
    app = QtWidgets.QApplication([])
    win = Widget()
    win.show()
    sys.exit(app.exec())
