const Router = Vaadin.Router;
const outlet = document.getElementById('outlet');
const router = new Router(outlet);

function PageSearch(options) {
  this.options = options;
  this.type = '';
}

PageSearch.prototype.index = function (type) {
  this.type = type;
  return this;
}

PageSearch.prototype.search = function (options) {
  return new Promise(async (resolve, reject) => {
    let response = await fetch(`/indexes/${this.type}/search`);
    let data = await response.json();
    resolve({
      hits: data
    });
  })
};

// modify by content
let is_pagedump = false;

const Quake = {
  // config for global
  pagedump: is_pagedump,
  flows: {},
  client: is_pagedump ? new PageSearch() : new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  }),
  router: router,
  transflow: {
    mapping: {},
    add: function (route) {
      let conf = {
        path: `/transflow/custom/${route.name}`,
        action: route.action
      };

      Quake.router.addRoutes(conf);

      const nav = document.createElement('a');
      nav.setAttribute("href", conf.path);
      nav.innerText = route.display;

      let navNode = document.getElementById("transflow-nav");
      navNode.appendChild(nav);
    }
  },
  query: function (entry, query, filter) {
    let index = Quake.client.index(entry);
    let options = {
      limit: 99,
      attributesToHighlight: ['overview']
    };

    // check filter
    Object.assign(options, filter);

    return index.search(query, options).then((result) => {
      return result.hits
    })
  }
}

window.Quake = Quake;

const home = (context, commands) => {
  const dashboard = document.createElement('quake-dashboard');
  dashboard.addEventListener("dispatchAction", function (e) {
    let define = e.detail;
    handleAction(define);
  });

  return dashboard
};

function create_editor_element(entry, params) {
  const editor = document.createElement('quake-editor');

  // todo: should set value first,?
  editor.setAttribute('value', entry.content);

  editor.setAttribute('id', entry.id);
  editor.setAttribute('title', entry.title);
  editor.setAttribute('entry_type', params.type);
  editor.addEventListener("onSave", function (event) {
    let data = event.detail;
    update_entry(params.type, params.id, {
      title: data.title,
      content: data.value.replaceAll("\\\n", "\n")
    })
  });
  return editor
}

const edit_entry = async (context, commands) => {
  let params = context.params;
  let response = await fetch(`/entry/${params.type}/${params.id}`)
  const entry = await response.json();

  return create_editor_element(entry, params);
}

const create_entry = async (context, commands) => {
  let params = context.params;
  let url_params = new URLSearchParams(context.search);
  let text = url_params.get('text');

  let url = `/entry/${params.type}?text=${text}`
  const response = await fetch(url, {
    method: 'POST'
  });

  const entry = await response.json();
  params.id = entry.id;

  return create_editor_element(entry, params);
}

const update_entry = async (entry_type, id, properties) => {
  let response = await fetch(`/entry/${entry_type}/${id}`, {
    method: 'POST',
    body: JSON.stringify({properties: properties})
  });

  let data = await response.json();
  console.log(data);
}

const show_entry = async (context, commands) => {
  let params = context.params;
  let response = await fetch(`/entry/${params.type}/${params.id}`)
  const entry = await response.json();

  const editor = document.createElement('quake-render');
  editor.setAttribute('content', entry.content);

  editor.addEventListener("clickPageLink", function (event) {
    let data = event.detail;
    Router.go(`/show/${data.type}/${data.id}`);
  });

  return editor
}

function handleAction(define) {
  if (define.action === 'add') {
    Router.go(`/entry/${define.entry}/new?text=${define.text}`)
  } else if (define.action === 'show') {
    Router.go(`/show/${define.entry}/${define.parameters[0]}`);
  } else if (define.parameters.length > 0) {
    Router.go(`/edit/${define.entry}/${define.parameters[0]}`);
  } else {
    console.log("some action");
  }
}

router.setRoutes([
  {path: '/', action: home},
  {path: '/entry/:type/new', action: create_entry},
  {path: '/edit/:type/:id', action: edit_entry},
  {path: '/show/:type/:id', action: show_entry},
]);
