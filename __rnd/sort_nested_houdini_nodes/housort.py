"""
import sys
import importlib
importpath = '/home/ford/storage/dev/Studies/__rnd/'
if importpath not in sys.path:
    sys.path = [importpath] + sys.path
import sort_nested_houdini_nodes.housort as  housort
importlib.reload(housort)
housort.main()
"""

import pprint
import hou

def main_00():
    root = hou.node('/obj/geo1/root')
    print(f':: root: {root}')

    print(f':: children:')
    print('\n'.join(map(str, root.children())))

    print(f':: allItems:')
    print('\n'.join(map(str, root.allItems())))

    print(f':: allNodes:')
    print('\n'.join(map(str, root.allNodes())))


def main():
    root = hou.node('/obj/geo1/root')
    kids = list_children(root)
    pprint.pprint(kids)

def list_children(root):
    def list_kids(node, kids):
        for kid in node.children():
            kids.append(kid)
            list_kids(kid, kids)
        return kids
    return list_kids(root, [])
