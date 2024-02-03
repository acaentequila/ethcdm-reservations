import { ConnectButton, ConnectDialog } from "@connect2ic/react"

const Login = () => {
	// const { isConnected, principal, activeProvider } = useConnect({
	// 	onConnect: () => {
	// 		// Signed in
	// 	},
	// 	onDisconnect: () => {
	// 		// Signed out
	// 	}
	// })

	return (
		<>
			<ConnectButton />
			<ConnectDialog dark={false} />
		</>
	)
}

export default Login
