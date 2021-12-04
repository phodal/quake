var Router = Vaadin.Router;
const outlet = document.getElementById('outlet');
const router = new Router(outlet);

const home = (context, commands) => {
  const dashboard = document.createElement('quake-dashboard');
  dashboard.addEventListener("dispatchAction", function (e) {
    let define = e.detail;

    if (define.action === 'add') {
      Router.go(`/entry/${define.object}/new?text=${define.text}`)
    } else if (define.parameters.length > 0) {
      Router.go(`/edit/${define.object}/${define.parameters[0]}`);
    } else {
      console.log("some action");
    }
  });

  return dashboard
};

function create_editor_element(entry, params) {
  const editor = document.createElement('quake-editor');

  // todo: should set value first,?
  editor.setAttribute('value', entry.content);

  editor.setAttribute('id', entry.id);
  editor.setAttribute('title', entry.title);
  editor.addEventListener("onSave", function (event) {
    update_entry(params.type, params.id, {
      title: event.detail.title,
      content: event.detail.value.replaceAll("\\\n", "\n")
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

  let url = `/entry/${params.type}/new?text=${text}`
  const response = await fetch(url, {
    method: 'POST'
  });

  const entry = await response.json();
  params.id = entry.id;
  return create_editor_element(entry, params);
}

const update_entry = async (entry_type, id, fields) => {
  let response = await fetch(`/entry/${entry_type}/${id}`, {
    method: 'POST',
    body: JSON.stringify({ fields: fields })
  });

  let data = await response.json();
  console.log(data);
}

router.setRoutes([
  {path: '/', action: home},
  {path: '/entry/:type/new', action: create_entry},
  {path: '/edit/:type/:id', action: edit_entry},
]);
