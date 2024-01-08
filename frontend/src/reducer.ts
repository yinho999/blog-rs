// src/reducer.ts
export const initialState = {
  user: null,
};

interface User {
  id: number;
  name: string;
  // add other user properties here
}

export interface State {
  user: User | null;
}
type LoginAction = { type: "LOGIN"; payload: User };
type LogoutAction = { type: "LOGOUT" };

export type Action = LoginAction | LogoutAction;
export function reducer(state: State, action: Action) {
  switch (action.type) {
    case "LOGIN":
      return {
        ...state,
        user: action.payload,
      };
    case "LOGOUT":
      return {
        ...state,
        user: null,
      };
    default:
      return state;
  }
}
