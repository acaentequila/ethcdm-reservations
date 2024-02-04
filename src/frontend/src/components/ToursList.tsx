import { useCanister } from "@connect2ic/react"
import { useEffect, useState } from "react"
import { Tour } from "../types/tour"
import TourItem from "./TourItem"

const ToursList: React.FC = () => {
	const [backend] = useCanister('backend')
	const [tours, setTours] = useState<Tour[] | undefined>()

	// read all tours 
	useEffect(() => {
		getTours()
	}, [])

	const getTours = async () => {
		let res = (await backend.listTours()) as [any, Tour][]
		console.log({ res })
		let tours = res.map((tour) => tour[1])

		setTours(tours as Tour[])
	}

	// render a TourItem for each one of them
	return <>
		{tours
			? tours.map((tour, index) => <TourItem key={index} tour={tour} />)
			: <>Loading...</>
		}

	</>
}

export default ToursList

