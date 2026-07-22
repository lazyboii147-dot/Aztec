namespace AztecOntology.Models
{
    public enum Glyph
    {
        Cipactli,
        Ehecatl,
        Calli,
        Cuetzpalin,
        Coatl,
        Miquiztli,
        Mazatl,
        Tochtli,
        Atl,
        Itzcuintli,
        Ozomahtli,
        Malinalli,
        Acatl,
        Ocelotl,
        Cuauhtli,
        Cozcacuauhtli,
        Ollin,
        Tecpatl,
        Quiahuitl,
        Xochitl
    }

    public enum Deity
    {
        Ometeotl,
        Quetzalcoatl,
        Tezcatlipoca,
        Huitzilopochtli,
        Tlaloc,
        Tonatiuh,
        Coyolxauhqui,
        Coatlicue,
        Chalchiuhtlicue,
        Xochiquetzal,
        Tlazolteotl,
        Mayahuel,
        Centeotl,
        Mictlantecuhtli,
        Xolotl,
        Xiuhtecuhtli,
        Huehuecoyotl,
        CentzonMimixcoa,
        CentzonHuitznahua,
        CentzonTotochtin
    }
}
public class AztecClient
{
    private readonly HttpClient _http;

    public AztecClient(HttpClient http) => _http = http;

    public async Task<GlyphInfo> GetGlyphAsync(Glyph glyph)
    {
        var resp = await _http.GetAsync($"/glyph/{glyph}");
        resp.EnsureSuccessStatusCode();
        return await resp.Content.ReadFromJsonAsync<GlyphInfo>();
    }
}
