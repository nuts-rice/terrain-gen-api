import init, { serve_heightmap } from 
async function run() {
  await init();
  //const _form = 
  const heightmap = serve_heightmap(_form);

  const canvas = document.getElementById('heightmapCanvas');
  const ctx = canvas.getContext('2d');
}

function renderHeightmap(_heightmap, processType, elementId, processFunction) {
  let itemsMeta = []
  let placeholder = "<div>"
  for (let i = 0; i < _heightmap.length; i++) {
    . . .
  }
  placeholder += "</div>"
  document.getElementById(elementId).innerHTML = placeholder;
}
