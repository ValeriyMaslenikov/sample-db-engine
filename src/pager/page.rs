enum PageType {
    TableNonLeafPage,
    TableLeafPage,
}

struct PageHeader {
    node_type: PageType,
}