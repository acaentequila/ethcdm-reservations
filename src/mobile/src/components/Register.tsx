import { Box, Button, Input, Label, Text } from "theme-ui"
import { FormEvent, useState } from "react"
import { useCanister } from "@connect2ic/react"
import { useTour } from "../context/tour.provider"

const Register: React.FC = () => {
	const { setTour } = useTour()
	const [backend] = useCanister('backend')
	const [title, setTitle] = useState('')
	const [price, setPrice] = useState(0)

	const handleSubmit = async (e: FormEvent) => {
		e.preventDefault()

		await backend.registerTour(title, price)

		alert('success')
		setTour({ title, price: price.toString() })
	}

	return <Box as="form" sx={{ padding: '20px' }} onSubmit={handleSubmit}>
		<Label>
			<Text>Name</Text>
			<Input onInput={(e) => setTitle(e.currentTarget.value)}></Input>
		</Label>
		<Label>
			<Text>Price</Text>
			<Input type="number" onInput={(e) => setPrice(Number(e.currentTarget.value))}></Input>
		</Label>

		<Button type="submit">Regiter</Button>
	</Box>
}

export default Register
