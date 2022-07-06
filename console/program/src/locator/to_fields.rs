// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<N: Network> ToFields for Locator<N> {
    type Field = Field<N>;

    /// Returns this locator as a list of field elements.
    fn to_fields(&self) -> Result<Vec<Self::Field>> {
        let mut fields = self.id.to_fields()?;
        if let Some(resource) = &self.resource {
            fields.push(resource.to_field()?);
        }
        Ok(fields)
    }
}
