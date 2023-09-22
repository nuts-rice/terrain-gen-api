import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import { TransformControls } from "three/addons/controls/TransformControls.js";
import { GUI } from "three/addons/libs/lil-gui.module.min.js";
let stats;

main();
function main() {
  try {
    var canvas = document.querySelector("#main");
    var renderer = new THREE.WebGLRenderer({ antialias: true, canvas });
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);
    var camera = new THREE.PerspectiveCamera(
      30,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    var clock = new THREE.Clock();
    var orbit = new OrbitControls(camera, renderer.domElement);

    camera.position.z = 100;
    orbit.update();
    var scene = new THREE.Scene();
    scene.background = new THREE.Color(0x232634);
    var geometry = new THREE.PlaneGeometry(20, 20, 20, 20);
    // var data = new HeightmapState;
    // var colors =  data.colors;
    // console.log(colors);
    var loader = new THREE.TextureLoader();
    var img = new Image();
    img.onload = function () {
  
     //get height data from img
     var data = getHeightData(img);
  
     // plane
     var geometry = new THREE.PlaneGeometry(10,10,9,9);
     var texture = THREE.ImageUtils.loadTexture('images/heightmap2.png' );
     var material = new THREE.MeshLambertMaterial( { map: texture } );
     plane = new THREE.Mesh( geometry, material );
     
     //set height of vertices
     for ( var i = 0; i<plane.geometry.vertices.length; i++ ) {
          plane.geometry.vertices[i].z = data[i];
     }

     scene.add(plane);
   
 };
    img.src = "static/images/heightmap_test.png";
    loader.load("static/images/heightmap_test.png", (texture) => {
      console.log(texture);
      texture.wrapS = THREE.ClampToEdgeWrapping;
      texture.wrapT = THREE.ClampToEdgeWrapping;
      texture.colorSpace = THREE.SRGBColorSpace;
      var material = new THREE.MeshPhongMaterial({
        color: 0xa6d189,
        side: THREE.DoubleSide,
        displacementMap: texture,
        displacementScale: 40,
      });

      // var wireframe = new THREE.WireframeGeometry(geometry);
      // var line = new THREE.LineSegments(wireframe);
      // line.material.depthTest = false;
      // line.material.opacity = 0.25;
      // line.material.transparent = true;

      var plane = new THREE.Mesh(geometry, material);
      scene.add(plane);
      // scene.add(line);
      // document.body.appendChild(stats.dom)
      var controls = new TransformControls(camera, renderer.domElement);
      controls.addEventListener("change", render);

      controls.addEventListener("dragging-changed", function (event) {
        orbit.enabled = !event.value;
      });
      controls.attach(plane);
      scene.add(controls);
      window.addEventListener("keydown", function (event) {
        switch (event.keyCode) {
          case 81: // Q
            controls.setSpace(controls.space === "local" ? "world" : "local");
            break;

          case 16: // Shift
            controls.setTranslationSnap(100);
            controls.setRotationSnap(THREE.MathUtils.degToRad(15));
            controls.setScaleSnap(0.25);
            break;

          case 87: // W
            controls.setMode("translate");
            break;

          case 69: // E
            controls.setMode("rotate");
            break;

          case 82: // R
            controls.setMode("scale");
            break;

          // case 67: // C
          //   const position = currentCamera.position.clone();

          //   currentCamera = currentCamera.isPerspectiveCamera
          //     ? cameraOrtho
          //     : cameraPersp;
          //   currentCamera.position.copy(position);

          //   orbit.object = currentCamera;
          //   controls.camera = currentCamera;

          //   currentCamera.lookAt(
          //     orbit.target.x,
          //     orbit.target.y,
          //     orbit.target.z
          //   );
          //   onWindowResize();
          //   break;

          case 86: // V
            const randomFoV = Math.random() + 0.1;
            const randomZoom = Math.random() + 0.1;

            cameraPersp.fov = randomFoV * 160;
            cameraOrtho.bottom = -randomFoV * 500;
            cameraOrtho.top = randomFoV * 500;

            cameraPersp.zoom = randomZoom * 5;
            cameraOrtho.zoom = randomZoom * 5;
            onWindowResize();
            break;

          case 187:
          case 107: // +, =, num+
            controls.setSize(controls.size + 0.1);
            break;

          case 189:
          case 109: // -, _, num-
            controls.setSize(Math.max(controls.size - 0.1, 0.1));
            break;

          case 88: // X
            controls.showX = !controls.showX;
            break;

          case 89: // Y
            controls.showY = !controls.showY;
            break;

          case 90: // Z
            controls.showZ = !controls.showZ;
            break;

          case 32: // Spacebar
            controls.enabled = !controls.enabled;
            break;

          case 27: // Esc
            controls.reset();
            break;
        }
      });
      render();
      animate();
      initLighting();
    });
    function initTexture(data, width, height) {
      let context, image, imageData, shade;
    }

    function color_terrain() {}
    function initGui() {
      const gui = new GUI();
    }

    function update() {
      var delta = clock.getDelta();
      var moveDistance = 50 * delta;
    }

     function getHeightData(img,scale) {
     
     if (scale == undefined) scale=1;
     
        var canvas = document.createElement( 'canvas' );
        canvas.width = img.width;
        canvas.height = img.height;
        var context = canvas.getContext( '2d' );

        var size = img.width * img.height;
        var data = new Float32Array( size );

        context.drawImage(img,0,0);

        for ( var i = 0; i < size; i ++ ) {
            data[i] = 0
        }

        var imgd = context.getImageData(10, 20, img.width, img.height);
        var pix = imgd.data;

        var j=0;
        for (var i = 0; i<pix.length; i +=4) {
            var all = pix[i]+pix[i+1]+pix[i+2];
            data[j++] = all/(12*scale);
        }
        
        return data;
    }
    function animate() {
      requestAnimationFrame(animate);
      orbit.update();
      renderer.render(scene, camera);
    }
    function initLighting() {
      var light = new THREE.DirectionalLight(0xffffff, 1);
      light.position.set(0, 1, 0);
      scene.add(light);

      var light = new THREE.DirectionalLight(0xffffff, 0.5);
      light.position.set(0, -1, 0);
      scene.add(light);

      var light = new THREE.DirectionalLight(0xffffff, 1);
      light.position.set(1, 0, 0);
      scene.add(light);

      var light = new THREE.DirectionalLight(0xffffff, 0.5);
      light.position.set(0, 0, 1);
      scene.add(light);

      var light = new THREE.DirectionalLight(0xffffff, 1);
      light.position.set(0, 0, -1);
      scene.add(light);

      var light = new THREE.DirectionalLight(0xffffff, 0.5);
      light.position.set(-1, 0, 0);
      scene.add(light);

      var ambientLight = new THREE.AmbientLight(0x555555);
      scene.add(ambientLight);
    }

    function resizeRendererToDisplaySize(renderer) {
      var canvas = renderer.domElement;
      var width = canvas.clientWidth;
      var height = canvas.clientHeight;

      var needResize = canvas.width !== width || canvas.height !== height;
      if (needResize) {
        renderer.setSize(width, height, false);
      }
      return needResize;
    }

    function render() {
      if (resizeRendererToDisplaySize(renderer)) {
        var canvas = renderer.domElement;
        camera.aspect = canvas.clientWidth / canvas.clientHeight;
        camera.updateProjectionMatrix();
      }
      const time = -performance.now() * 0.0003;
      camera.position.x = 400 * Math.cos(time);
      camera.position.z = 400 * Math.sin(time);
      camera.lookAt(scene.position);

      renderer.render(scene, camera);

      requestAnimationFrame(render);
    }

    requestAnimationFrame(render);
  } catch (error) {
    console.error("error in main ", error);
  }
}
//TODO: load black and white png heightmap
//let scene, camera, renderer, plane, light;
//main();
//async function main() {
//  var canvas = document.querySelector("main");
//  init_scene();
//  renderScene(renderer);

//  async function init_scene() {
//    let textureLoader = new THREE.TextureLoader();
//    try {
//      let heightMap = await loadTexture(
//        textureLoader,
//        "static/images/heightmap_test.png"
//      );
//      //TODO: Load Image and display on canvas somehow
//      // var heightMapImage = new
//      heightMap.wrapS = heightMap.wrapT = THREE.RepeatWrapping;
//      heightMap.repeat.set(100, 100);
//      // var MAX_POINTS = 500;
//      scene = new THREE.Scene();
//      camera = init_camera();
//      camera.position.z = 100;
//      var renderer = init_renderer();
//      light = init_light();
//      scene.add(light);
//      console.log(light);
//      plane = init_geometry(heightMap);
//      scene.add(plane);
//      console.log(plane);
//    } catch (error) {
//      console.error("An error occurred while initializing the scene:", error);
//    }
//  }

//  function renderScene(renderer) {
//    if (resizeRenderer(renderer)) {
//      var canvas = renderer.domElement;
//      camera.aspect = canvas.clientWidth / canvas.clientHeight;
//      camera.updateProjectionMatrix();
//    }
//    renderer.render(scene, camera);
//    requestAnimationFrame(renderScene);
//  }

//  function init_camera() {
//    return new THREE.PerspectiveCamera(
//      75,
//      window.innerWidth / window.innerHeight,
//      0.1,
//      1000
//    );
//  }
//  function init_renderer() {
//    let renderer = new THREE.WebGLRenderer({ antialias: true });
//    renderer.setSize(window.innerWidth, window.innerHeight);
//    document.body.appendChild(renderer.domElement);
//    return renderer;
//  }

//  function resizeRenderer(renderer) {
//    var canvas = renderer.domElement;
//    var width = canvas.clientWidth;
//    var height = canvas.clientHeight;
//    var needResize = canvas.width !== width || canvas.height !== height;
//    if (needResize) {
//      renderer.setSize(width, height, false);
//    }
//    return needResize;
//  }

//  function init_light() {
//    var color = 0xffffff;
//    var intensity = 1;
//    let light = new THREE.DirectionalLight(color, intensity);
//    light.position.set(-1, 2, 4);
//    return light;
//  }

//  function init_geometry(texture) {
//    var geometry = new THREE.PlaneGeometry(5, 5, 5, 5);
//    var material = new THREE.MeshPhongMaterial({
//      color: 0xffff00,
//      side: THREE.DoubleSide,
//      displacementMap: texture,
//      displacementScale: 20,
//    });
//    let plane = new THREE.Mesh(geometry, material);
//    plane.rotation.x = Math.PI / 2;
//    return plane;
//  }

//  function loadTexture(loader, url) {
//    return new Promise((resolve, reject) => {
//      loader.load(url, resolve, undefined, reject);
//    });
//  }
//}
// function update_heightmap(){
//   var positions = geometry.attributes.position.array;

// }
document.querySelector("form").addEventListener("render", function (e) {
  e.preventDefault();
  var data = new FormData(e.target);
  fetch("/post_input", {
    method: "POST",
    body: data,
  })
    .then((response) => response.json())
    .then((data) => {
      // var noise = new Float32Array(data);
      // for (let i = 0; i < positions.length; i++) {
      //   positions[i] += noise[i];
      // }
      // geometry.setAttribute(
      //   "position",
      //   new THREE.BufferAttribute(positions, 1)
      // );
      // console.log(geometry.getAttribute.postion);
      // geometry.attributes.position.neesUpdate = true;
      // animate();
    });
});

// document.querySelector("#web_heightmap").src = data.url;
