import { FormEvent, useState } from "react";

interface SearchBarProps {
  onSearch: (ticker: string) => void;
}

function SearchBar({ onSearch }: SearchBarProps) {
  const [searchTerm, updateSearchTerm] = useState<string>("");
  const handleInput = (event: React.ChangeEvent<HTMLInputElement>) => {
    let { value } = event.target;
    updateSearchTerm(value);
  };
  const handleSubmit = (event: FormEvent) => {
    event.preventDefault();
    onSearch(searchTerm);
  };

  return (
    <header className="bg-green-400 py-4 h-20 flex items-center justify-around">
      <div className="font-bold text-2xl">BullsEye</div>
      <div className="bg-white rounded-xl border border-black">
        <form onSubmit={handleSubmit}>
          <input
            type="text"
            name="ticker"
            value={searchTerm}
            placeholder="Search Ticker"
            onChange={handleInput}
            className="ps-3"
          />
        </form>
      </div>
    </header>
  );
}

export default SearchBar;
