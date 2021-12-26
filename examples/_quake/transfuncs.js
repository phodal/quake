function from_quake_references_to_network(data) {
  let category_index = [];
  let nodes = [];
  let links = [];

  for (let key of Object.keys(data)) {
    let node = data[key];

    if (!category_index.includes(node.source_type)) {
      category_index.push(node.source_type);
    }
    nodes.push({
      id: parseInt(node.source_id),
      name: node.source_title,
      category: category_index.indexOf(node.source_type)
    });

    for (let ref of node.references) {
      if (!category_index.includes(ref.entry_type)) {
        category_index.push(ref.entry_type);
      }

      nodes.push({
        id: parseInt(ref.entry_id),
        name: ref.entry_title,
        category: category_index.indexOf(ref.entry_type)
      })

      links.push({source: parseInt(node.source_id) + "", target: parseInt(ref.entry_id) + ""});
    }
  }

  let categories = [];
  for (let category of category_index) {
    categories.push({name: category})
  }
  return {
    nodes,
    links,
    categories
  }
}

const tl_show_network = async (context, commands) => {
  const el = document.createElement('graph-network');

  let response = await fetch("/reference/quake_book");
  let data = from_quake_references_to_network(await response.json());
  el.data = data;

  return el;
}

Quake.flows['tl_show_network'] = tl_show_network;
