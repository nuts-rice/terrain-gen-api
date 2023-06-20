import * as THREE from "./node_modules/three/build/three.module.js";
console.log(THREE)
document.querySelector("form").addEventListener("render", function (e) {
  e.preventDefault();
  var data = new FormData(e.target);
  fetch("/post_input", {
    method: "POST",
    body: data,
  })
    .then((response) => response.json())
    .then(data => {
      // document.querySelector("#web_heightmap").src = data.url;
      console.log(data);
    });
});

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(
  75,
  window.innerWidth / window.innerHeight,
  0.1,
  1000
);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);
const geometry = new THREE.PlaneGeometry(1,1);
const material = new THREE.MeshBasicMaterial( {color: 0xffff00, side: THREE.DoubleSide} );
const plane = new THREE.Mesh( geometry, material );
scene.add( plane );
camera.position.z = 5;
function animate() {
	requestAnimationFrame( animate );
  plane.rotation.x += 0.01;
	plane.rotation.y += 0.01;
	renderer.render( scene, camera );
}
animate();

