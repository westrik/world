import { assign, Machine } from 'xstate';
import { Session } from '../SignInForm';

export const SESSION_KEY = 'api_session';

interface Context {
  session: Session | null;
  persistLogin: boolean;
}

function loadSession(): Session | null {
  const session =
    sessionStorage.getItem(SESSION_KEY) || localStorage.getItem(SESSION_KEY);
  if (session) {
    return JSON.parse(session) as Session;
  }
  return null;
}

// TODO: Move to SignInForm machine
function storeSession(ctx: Context): void {
  // tslint:disable-next-line:no-console
  console.log('storing session');
}

function clearSession(): void {
  // tslint:disable-next-line:no-console
  console.log('clearing session');
  sessionStorage.clear();
  localStorage.clear();
}

export const appMachine = Machine<Context>({
  id: 'app-container',
  initial: 'loading',

  context: {
    session: loadSession(),
    persistLogin: false,
  },

  states: {
    loading: {
      on: {
        '': [
          {
            target: 'signedOut',
            cond: ({ session }): boolean => session === null,
          },
          {
            target: 'signedIn',
            cond: ({ session }): boolean => session !== null,
          },
        ],
      },
    },
    signedOut: {
      entry: clearSession,
    },
    signedIn: {},
  },

  on: {
    SIGN_OUT: {
      target: 'signedOut',
    },
    SIGN_IN: {
      target: 'signedIn',
    },
  },
});

// interface TodoCtx {
//   completed: boolean;
// }
// interface TodosCtx {
//   todos: Array<Interpreter<TodoCtx>>;
// }
//
// const todoMachine = Machine<TodoCtx>({
//   context: {
//     completed: false
//   },
//   initial: 'uncompleted',
//   states: {
//     uncompleted: {
//       on: {
//         COMPLETE: 'done'
//       }
//     },
//     done: {
//       entry: assign<TodoCtx>({ completed: true })
//     }
//   }
// });
//
// interface Event {
//   type: 'CREATE';
// }
//
// const todosMachine = Machine<TodosCtx, StateSchema<TodosCtx>, Event>({
//   context: { todos: [] },
//   initial: 'working',
//   states: { working: {} },
//   on: {
//     CREATE: {
//       actions: assign(ctx => ({
//         ...ctx,
//         todos: ctx.todos.concat(spawn(todoMachine))
//       }))
//     }
//   }
// });
//
// const service = interpret(todosMachine).start();
// service.send('CREATE');
