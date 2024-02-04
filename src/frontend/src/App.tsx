import { Card } from "theme-ui"
import Navbar from "./components/Navbar"
import ToursList from "./components/ToursList"

function App() {

	return (
		<>
			<Navbar />
			<Card sx={{ margin: '100px 400px', padding: '40px' }}>

				<h1>Reservations</h1>
				<h2>Things to implement</h2>
				<ul>
					<li>List of tours and a modal for each one of them</li>
					<li>List of reservations</li>
				</ul>

				<ToursList />
			</Card>
		</>
	)
}

export default App
