<script lang="ts">
	interface Props {
		headers: string[];
		rows: (string | number | boolean)[][];
		caption?: string;
	}

	let { headers, rows, caption }: Props = $props();
</script>

<table>
	{#if caption !== undefined}
		<caption>{caption}</caption>
	{/if}

	<thead>
		<tr>
			{#each headers as header}
				<th>{header}</th>
			{/each}
		</tr>
	</thead>

	<tbody>
		{#each rows as row}
			<tr>
				{#each row as cell, index}
					<td data-cell={headers[index]}>{cell}</td>
				{/each}
			</tr>
		{/each}
	</tbody>
</table>

<style>
	table {
		background-color: var(--primary-bg);
		border-collapse: collapse;
		color: var(--primary-text);
		padding: 1rem;
		margin-inline: auto;
		min-width: 200px;
		width: min(900px, 100% - 3rem);
	}

	thead > tr > th {
		background-color: var(--quaternary-bg);
	}

	caption,
	th,
	td {
		font-size: 1.25rem;
		padding: 1rem;
	}

	caption,
	th {
		text-align: left;
	}

	caption {
		background-color: var(--primary-bg);
		border-radius: 5px 5px 0 0;
		font-size: 1.5rem;
		font-weight: 700;
	}

	tr:nth-of-type(2n) {
		background-color: var(--tertiary-bg);
	}

	@media (max-width: 650px) {
		th {
			display: none;
		}

		td {
			display: grid;
			gap: 0.5rem;
			grid-template-columns: 15ch auto;
			padding: 0.5rem 1rem;
		}

		td:first-child {
			padding-top: 2rem;
		}

		td:last-child {
			padding-bottom: 2rem;
		}

		td::before {
			content: attr(data-cell) ': ';
			font-weight: 700;
		}
	}
</style>
