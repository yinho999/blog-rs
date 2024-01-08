// src/context/AppContext.tsx
import { createContext } from 'react';
import { State, Action } from '../reducer'; // adjust the path as needed

interface AppContextProps {
  state: State;
  dispatch: React.Dispatch<Action>;
}

export const AppContext = createContext<AppContextProps | undefined>(undefined);