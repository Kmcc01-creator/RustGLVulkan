// ui/src/components/EngineInterface.tsx
import { invoke } from '@tauri/api'

interface EngineData {
    memoryStats: MemoryStats;
    graphicsStats: GraphicsStats;
}

const EngineInterface: React.FC = () => {
    const [engineData, setEngineData] = useState<EngineData>();

    useEffect(() => {
        // Listen for updates from Rust
        const unlisten = listen('engine-update', (event) => {
            setEngineData(event.payload);
        });

        return () => {
            unlisten();
        };
    }, []);

    const sendCommand = async (command: string) => {
        await invoke('handle_debug_command', { command });
    };

    return (
        <div>
            <MemoryVisualizer data={engineData?.memoryStats} />
            <GraphicsDebugger data={engineData?.graphicsStats} />
            <DebugControls onCommand={sendCommand} />
        </div>
    );
}