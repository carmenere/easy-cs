import logging
import os
from typing import Any


class CustomFormatter(logging.Formatter):
    ESC = "\x1b"
    CSI = f"{ESC}["
    dark_grey = f"{CSI}38;5;240m"
    light_grey = f"{CSI}38;5;247m"
    green = f"{CSI}38;5;34m"
    yellow = f"{CSI}38;5;208m"
    red = f"{CSI}38;5;196m"
    pink = f"{CSI}38;5;201m"
    white = f"{CSI}38;5;255m"
    reset = f"{CSI}0m"
    log_fmt = '[[%(asctime)s::%(levelname)s::%(pathname)s::%(lineno)s::%(funcName)s]] %(message)s'

    FORMATS = {
        logging.DEBUG: dark_grey + log_fmt + reset,
        logging.INFO: white + log_fmt + reset,
        logging.WARNING: yellow + log_fmt + reset,
        logging.ERROR: red + log_fmt + reset,
        logging.CRITICAL: pink + log_fmt + reset
    }

    def format(self, record):
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)


severity = os.environ.get("SEVERITY", None) or 'info'
severity = getattr(logging, severity.upper())

custom_fmt = logging.StreamHandler()
custom_fmt.setLevel(severity)
custom_fmt.setFormatter(CustomFormatter())


LOG = logging.getLogger(__name__)
LOG.propagate=False
LOG.addHandler(custom_fmt)
LOG.setLevel(severity)


class TSort:
    def __init__(self, edges):
        self.edges = edges
        self.deps = {}
        self.nodes = set([])
    
    def infer_deps(self, skip_loops):
        # (L, R) means L depends from R: L -> R
        for lnode, rnode in self.edges:
            if rnode is None:
                self.deps.setdefault(lnode, set([]))
            else:
                if skip_loops and lnode == rnode:
                    # Skip loop. Loop is an edge that connects a vertex to itself.
                    continue
                self.deps.setdefault(lnode, set([])).add(rnode)
                self.deps.setdefault(rnode, set([]))

    def is_not_empty(self, l):
        if l:
            return True
        else:
            return False
    
    def tsort(self, skip_loops):
        order = []
        next = True
        self.infer_deps(skip_loops)
        LOG.debug(f"self.deps: {self.deps}")
        while next:
            deleted = set([])
            new_deps = {}

            # Independent nodes are selected here and marked as deleted.
            for nname, ndeps in self.deps.items():
                if not ndeps:
                    order.append(nname)
                    deleted.add(nname)
            
            # Nodes that were marked as deleted are removed from self.deps and from every dependency set of any other node.
            for nname, ndeps in self.deps.items():
                if ndeps:
                    new_deps.setdefault(nname, ndeps - deleted)
            
            self.deps = new_deps

            LOG.debug(f"self.deps: {self.deps}, deleted: {deleted}")
            
            # Stop 'while loop' if set 'deleted' became empty.
            next = self.is_not_empty(deleted)
        
        if self.deps:
            msg = "There are loops in deps!"
            LOG.error(msg)
            return []
        
        return order

test_cases = [
    [('A', 'B'), ('A', 'D'), ('B', 'D'), ('D', 'C'), ('A', 'C'), ('X', 'R')],
    [('A', 'B'), ('A', 'D'), ('B', 'D'), ('D', 'C'), ('A', 'C'), ('C', 'A'), ('X', 'R')]
]

def run_tests(test_cases):
    for tcase in test_cases:
        ordered = TSort(tcase).tsort(skip_loops=True)
        if ordered:
            LOG.info(f"Linearized order: {ordered}")

run_tests(test_cases)
