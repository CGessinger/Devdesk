import type { SettingsModel } from "$src/Settings/utils/SettingsModel";
import * as THREE from "three";

export class SpaceBackground {
    readonly canvas;
    readonly active: boolean;
    readonly color: boolean;
    readonly particlesCount = 2000;

    constructor(_canvas, _settings: SettingsModel) {
        this.canvas = _canvas;
        this.active = _settings.runThree;
        this.color = _settings.dark_mode;
    }

    setup(): Function {
        if(!this.active)
            return () => {};

        const scene = new THREE.Scene()

        const positions = new Float32Array(this.particlesCount * 3);
        const color = new Float32Array(this.particlesCount * 3);

        for(let i = 0; i < this.particlesCount * 3; i++) {
            positions[i] = (Math.random() - 0.5) * 10;
            color[i] = Math.random();
        }
        const particlesGeometry = new THREE.BufferGeometry();
        particlesGeometry.setAttribute(
            "position", 
            new THREE.BufferAttribute(positions, 3)
        );
        particlesGeometry.setAttribute(
            "color", 
            new THREE.BufferAttribute(color, 3)
        );

        const particlesMaterial = new THREE.PointsMaterial({
            sizeAttenuation: true,
            size: 0.05,
            vertexColors: true
        });

        const particles = new THREE.Points(particlesGeometry, particlesMaterial);
        scene.add(particles);

        const sizes = {
            width: this.canvas.getBoundingClientRect().width,
            height: this.canvas.getBoundingClientRect().height
        }

        const camera = new THREE.PerspectiveCamera(35, sizes.width / sizes.height, 0.1, 100);
        camera.position.z = 3;
        scene.add(camera)

        const directionalLight = new THREE.DirectionalLight('#ffffff', 1)
        directionalLight.position.set(1, 1, 0)
        scene.add(directionalLight)

        const cursor = new THREE.Vector2;
        cursor.set(0, 0);

        window.addEventListener("mousemove", (e) => {
            cursor.x = e.clientX / sizes.width - 0.5;
            cursor.y = e.clientY / sizes.height - 0.5;
        });

        const renderer = new THREE.WebGLRenderer({
            canvas: this.canvas,
            alpha: true
        })
        renderer.setSize(sizes.width, sizes.height, false);
        renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))

        const clock = new THREE.Clock()
        let previousTime = 0;
        const tick = () =>
        {
            const elapsedTime = clock.getElapsedTime()
            const deltaTime = elapsedTime - previousTime;
            previousTime = elapsedTime;

            const parallaxX = cursor.x * 0.5;
            const parallaxY = - cursor.y * 0.5;
            camera.position.x += (parallaxX - camera.position.x) * 5 * deltaTime;
            camera.position.y += (parallaxY - camera.position.y) * 5 * deltaTime;
            camera.position.z -= (deltaTime * 0.1);
            
            // Render
            renderer.render(scene, camera)

            // Call tick again on the next frame
            window.requestAnimationFrame(tick)
        }

        return tick;
    }
}