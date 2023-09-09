class Node:
    def __init__(self, name):
        self.name = name
        self.children = []
    def __repr__(self):
        return f'<{self.name}>'


node31 = Node('level 3, node 1')
node32 = Node('level 3, node 2')
node33 = Node('level 3, node 3')

node21 = Node('level 2, node 1')
node21.children = [
    node31,
    node32,
    node33,
]
node22 = Node('level 2, node 2')
node23 = Node('level 2, node 3')
node24 = Node('level 2, node 4')
node25 = Node('level 2, node 5')

root = Node('root')
root.children = [
    node21,
    node22,
    node23,
    node24,
    node25,
]

def print_kids(node):
    indent = 0
    def pr(node, indent):
        print('\t'*indent, node)
        indent += 1
        for kid in node.children:
            pr(kid, indent)
    pr(node, indent)

print_kids(root)
