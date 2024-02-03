import React from 'react'
import ReactDOM from 'react-dom/client'
import { ThemeUIProvider } from 'theme-ui'
import App from './App.tsx'
import { theme } from './theme.ts'
import ICProvider from './context/connect2ic.provider.tsx'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<ICProvider>
			<ThemeUIProvider theme={theme}>
				<App />
			</ThemeUIProvider>
		</ICProvider>
	</React.StrictMode>,
)
