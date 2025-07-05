import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";

interface SearchBarProps {
  onSearch: (ticker: string) => void;
}

function SearchBar({ onSearch }: SearchBarProps) {
  const [searchTerm, updateSearchTerm] = useState<string>("");
  const navigate = useNavigate();
  const handleInput = (event: React.ChangeEvent<HTMLInputElement>) => {
    let { value } = event.target;
    updateSearchTerm(value);
  };
  const handleSubmit = (event: FormEvent) => {
    event.preventDefault();
    onSearch(searchTerm);
    navigate("/search");
  };

  return (
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
  );
}

export default SearchBar;
