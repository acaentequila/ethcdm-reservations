import { useCanister, useConnect } from "@connect2ic/react"
import Connect from "./components/Connect"
import Scanner from "./components/Scanner"
import { useTour } from "./context/tour.provider"
import Register from "./components/Register"
import { useEffect } from "react"
import { Tour } from "./types/tour"

function App() {
	const [backend, { loading }] = useCanister('backend')
	const { isConnected, principal } = useConnect()
	const { tour, setTour } = useTour()

	useEffect(() => {
		if (isConnected) {
			fetchTour()
		}
	}, [])

	const fetchTour = async () => {
		const res = await backend.getTour(principal)
		console.log({ tour: res })
		if (!res) return console.log('No tour found')
		setTour(res as Tour)
	}

	if (loading) return <p>loading...</p>

	return (
		<>
			{!isConnected
				? <Connect />
				: !tour
					? <Register />
					: <Scanner />
			}
		</>
	)
}

export default App
