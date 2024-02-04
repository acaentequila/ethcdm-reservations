import { ConnectButton, ConnectDialog, useConnect } from "@connect2ic/react"
import { Flex } from "theme-ui"

const Navbar = () => {
	const { isConnected, principal } = useConnect({
		// onConnect: () => {
		// 	// Signed in
		// },
		// onDisconnect: () => {
		// 	// Signed out
		// }
	})

	return (
		<Flex sx={{ justifyContent: 'end', padding: '10px' }} as="nav">
			<Flex>
				{isConnected && <pre>{principal}</pre>}
				<ConnectButton />
			</Flex>

			<ConnectDialog dark={false} />
		</Flex>
	)
}

export default Navbar
