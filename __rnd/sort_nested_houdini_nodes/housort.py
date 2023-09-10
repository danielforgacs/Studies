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
import json
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


def main_01():
    root = hou.node('/obj/geo1/root')
    kids = list_children(root)

def list_children(root):
    def list_kids(node, kids, level):
        print('\t'*level, node)
        for kid in node.children():
            kids.append(kid)
            list_kids(kid, kids, level+1)
        return kids
    return list_kids(root, [], 0)



def main():
    root = hou.node('/obj/geo1/root')
    sort_kids(root)


def sort_kids(node):
    get_out = lambda x: hou.node(f'{x.path()}/output0')
    allnodes = []
    level = 0
    def link_inputs(node, allnodes, level):
        level += 1
        for node in node.inputs():
            print('\t'*level, f'{node}')
            allnodes.append(node)
            link_inputs(node, allnodes, level)
    link_inputs(get_out(node), allnodes, level)
