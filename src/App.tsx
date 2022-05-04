import { MantineProvider } from '@mantine/core';
import { homeDir } from '@tauri-apps/api/path';
import { useEffect, useState } from 'react';
import CarryDistributionForm from './components/CarryDistributionForm';

function App() {
  const [count, setCount] = useState(0);
  const [home, setHome] = useState('');

  useEffect(() => {
    homeDir().then(setHome);
  });

  return (
    <>
      <style>
        @import
        url('https://fonts.googleapis.com/css2?family=Roboto&display=swap');
      </style>
      <MantineProvider
        withGlobalStyles
        withNormalizeCSS
        theme={{
          // Override any other properties from default theme
          colorScheme: 'dark',
          fontFamily: 'Roboto, sans serif',
          spacing: { xs: 15, sm: 20, md: 25, lg: 30, xl: 40 },
        }}>
        <CarryDistributionForm />
      </MantineProvider>
    </>
  );
}

export default App;
