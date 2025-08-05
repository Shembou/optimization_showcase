import { useGraphQL } from "../hooks/graphql";

const GraphQL = () => {
    type TResult = {
        add: number,
        humans: {
            name: string,
            age: number
        }[]
    }
    const { loading, result } = useGraphQL<TResult>(`add(a:2,b:2)
  humans {
    name,
    age
  }`);

    return (
        <section>
            {!loading && <div className="grid">
                <p>{result?.humans[0].age}</p>
            </div>}
        </section>
    );
};

export default GraphQL;
