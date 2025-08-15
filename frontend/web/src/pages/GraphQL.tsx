import { useGraphQL } from "../hooks/graphql";

const GraphQL = () => {
    type TResult = {
        users: {
            id: number,
            name: string,
            language: string,
            bio: string
        }[]
    }
    const { loading, result } = useGraphQL<TResult>(`  users {
    id,
    name,
    language,
    bio
  }`);

    return (
        <section>
            {!loading && <div className="grid">
                <p>{result?.users[0].id}</p>
            </div>}
        </section>
    );
};

export default GraphQL;
