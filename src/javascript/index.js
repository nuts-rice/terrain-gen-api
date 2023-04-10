import init, { serve_heightmap } from 
async function run() {
  await init();
  //const _form = 
  const heightmap = serve_heightmap(_form);

  const canvas = document.getElementById('heightmapCanvas');
  const ctx = canvas.getContext('2d');
}
