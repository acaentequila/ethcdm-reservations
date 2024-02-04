import { ConnectButton, ConnectDialog } from "@connect2ic/react"
import { Flex } from "theme-ui"

const Connect: React.FC = () => {

	return <Flex>
		<ConnectButton />
		<ConnectDialog dark={false} />
	</Flex>
}

export default Connect
