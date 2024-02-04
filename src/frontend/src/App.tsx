import { Card } from "theme-ui"
import Navbar from "./components/Navbar"
import ToursList from "./components/ToursList"

function App() {

	return (
		<>
			<Navbar />
			<Card sx={{ margin: '100px 400px', padding: '40px' }}>

				<h1>Tours</h1>
				<ToursList />
			</Card>
		</>
	)
}

export default App
