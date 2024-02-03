import { createClient } from "@connect2ic/core"
import { Connect2ICProvider } from "@connect2ic/react"
import { AstroX } from "@connect2ic/core/providers/astrox"
import { PlugWallet } from "@connect2ic/core/providers/plug-wallet"
import "@connect2ic/core/style.css"
import { ReactNode } from "react"
import * as backend from '../../../declarations/backend'

console.log({
	backend,
	env: import.meta.env.DFX_VERSION
})

const client = createClient({
	canisters: {
		backend: {
			canisterId: backend.canisterId,
			// @ts-ignore
			idlFactory: backend.idlFactory
		}
	},
	providers: [
		new AstroX(),
		new PlugWallet()
	],
})

interface IProps {
	children: ReactNode
}
const ICProvider: React.FC<IProps> = ({ children }) => (
	<Connect2ICProvider client={client}>
		{children}
	</Connect2ICProvider>
)

export default ICProvider
