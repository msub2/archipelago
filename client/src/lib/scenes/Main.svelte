<script>
    import { T } from '@threlte/core'
    import { OrbitControls, Sky } from '@threlte/extras';
</script>


<T.Mesh rotation={[-Math.PI / 2, 0, 0]} userData={{ name: 'island1' }}>
    <T.MeshBasicMaterial color={0x00ff00}/>
    <T.CircleGeometry args={[256, 32]} />
    <!-- Test Box 1 -->
    <T.Mesh>
        <T.BoxGeometry />
        <T.MeshBasicMaterial />
    </T.Mesh>
</T.Mesh>
<T.Mesh position={[512, 0, 512]} rotation={[-Math.PI / 2, 0, 0]} userData={{ name: 'island2' }}>
    <T.MeshBasicMaterial color={0x00ff00}/>
    <T.CircleGeometry args={[128, 32]} />
    <!-- Test Box 2 -->
    <T.Mesh position={[0, 0, 32]} scale={[16, 16, 16]}>
        <T.BoxGeometry />
        <T.MeshBasicMaterial />
    </T.Mesh>
</T.Mesh>
<T.Mesh position={[0, -0.1, 0]} rotation={[-Math.PI / 2, 0, 0]} userData={{ name: 'ocean' }}>
    <T.MeshBasicMaterial color={0x0000ff}/>
    <T.PlaneGeometry args={[8192, 8192]} />
</T.Mesh>
<T.GridHelper />
<OrbitControls />
<Sky elevation={6} />
