/*
// source: https://github.com/davidkpiano/xstate/issues/369#issuecomment-466080061

const getUsers = state => state.entities.users
const receiveEntities = entities => ({ type: "RECEIVE_ENTITIES", entities })

const { users } = useMappedState(state => ({ users: getUsers(state) }))
const dispatch = useDispatch()

const [state, send] = useMachine(myMachine, {
    context: {
        users
    },
    actions: {
        mergeEntities: (context, event) => dispatch(receiveEntities(event.data.entities))
    }
})
*/

// TODO:
//  - store + manage application data in redux, wrap with xstate FSM UI layer
//  - lil wasms: http://cliffle.com/blog/bare-metal-wasm/
//  - get basic Redux + XState functionality working with TS+Rust+WASM
//  - port all XState + Redux + hooks logic to use new library
//  - add testing helpers (in Rust and TS layers)
//  - compile rust state management library into iOS / Android apps
