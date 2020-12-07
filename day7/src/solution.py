# python 3.9+
pip install graph-theory
from graph import Graph

g = Graph()
with open("input/test.txt") as file:
    for line in file.readlines():
        key, value = line.rstrip().split(" bags contain ")
        el: str
        for el in value.split(", "):
            el = el.removesuffix(".")
            el = el.removesuffix("s")
            el = el.removesuffix(" bag")
            n, *rest = el.split()
            try:
                n = int(n)
                g.add_edge(key, " ".join(rest), n, bidirectional=False)
            except:
                print("missing", key, value)
c = 0
for e in g.nodes():
    if e != "shiny gold":
        l = g.all_paths(e, "shiny gold")
        c += 1 if len(l) else 0
print(c)

c = 0
for e in g.nodes():
    if e != "shiny gold":
        l = g.all_paths("shiny gold", e)
        for el in l:
            m=1
            for i in range(len(el) - 1):
                m*=g.edge(el[i], el[i + 1], default=-1)
            c+=m
print(c)
