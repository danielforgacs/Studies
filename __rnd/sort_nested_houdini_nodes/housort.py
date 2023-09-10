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
    kids = sort_kids(root)
    for kid in kids:
        print(f'{kid}')

def sort_kids(node):
    get_out = lambda x: hou.node(f'{x.path()}/output0')
    allnodes = []
    level = 0
    def link_inputs(node, allnodes, level):
        input = node.inputs()
        if not input:
            return
        input = input[0]
        allnodes.append(input)
        link_inputs(input, allnodes, level)
    link_inputs(get_out(node), allnodes, level)
    for kid in allnodes:
        if kid.isNetwork():
            # sort_kids(kid)
            link_inputs(get_out(kid), allnodes, level+1)
    return list(reversed(allnodes))
